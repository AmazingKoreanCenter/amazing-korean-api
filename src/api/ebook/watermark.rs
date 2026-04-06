use image::{DynamicImage, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use sha2::{Digest, Sha256};
use std::sync::OnceLock;

use crate::error::{AppError, AppResult};

/// 워터마크 폰트 (한 번만 로드, 프로세스 수명과 동일)
static WATERMARK_FONT: OnceLock<Option<ab_glyph::FontArc>> = OnceLock::new();

/// 워터마크 폰트 초기화 (서버 시작 시 호출)
pub fn init_font(font_path: &str) {
    let font = std::fs::read(font_path)
        .ok()
        .and_then(|bytes| ab_glyph::FontArc::try_from_vec(bytes).ok());

    if font.is_none() {
        tracing::warn!("Watermark font not found at '{font_path}'. Footer watermark disabled, LSB + microdot only.");
    }

    let _ = WATERMARK_FONT.set(font);
}

/// 4중 비가시적 워터마크 적용:
/// 1. 풋터 워터마크 (구매코드를 풋터 영역에 자연스럽게 삽입)
/// 2. 마이크로 도트 (4 모서리에 user_id 비트 인코딩, 1-2px near-white)
/// 3. LSB 스테가노그래피 (purchase_code + watermark_id 해시)
/// 4. 접근 로그 (호출측 service.rs에서 처리)
pub fn apply_watermark(
    image_bytes: &[u8],
    purchase_code: &str,
    watermark_id: &str,
    user_id: i64,
    page_num: i32,
) -> AppResult<Vec<u8>> {
    // WebP 디코드
    let img = image::load_from_memory(image_bytes)
        .map_err(|e| AppError::Internal(format!("Failed to decode image: {e}")))?;

    let mut rgba_img = img.to_rgba8();

    // 1. 풋터 워터마크 (폰트 있을 때만)
    if let Some(Some(font)) = WATERMARK_FONT.get() {
        apply_footer_watermark(&mut rgba_img, purchase_code, page_num, font);
    }

    // 2. 마이크로 도트 (user_id 비트 인코딩)
    apply_microdot_watermark(&mut rgba_img, user_id);

    // 3. LSB 스테가노그래피 (포렌식 히든)
    apply_lsb_watermark(&mut rgba_img, purchase_code, watermark_id);

    // WebP 인코딩
    let dynamic = DynamicImage::ImageRgba8(rgba_img);
    let mut buf = std::io::Cursor::new(Vec::new());
    dynamic
        .write_to(&mut buf, image::ImageFormat::WebP)
        .map_err(|e| AppError::Internal(format!("Failed to encode watermarked image: {e}")))?;

    Ok(buf.into_inner())
}

/// 풋터 워터마크: 페이지 하단 풋터 영역에 구매코드를 자연스럽게 삽입
/// 기존 풋터: "{pageNum}  Amazing Korean" → "{pageNum} | {purchaseCode} | Amazing Korean"
/// 텍스트 폭에 딱 맞는 영역만 클리어 (기존 풋터 양식 보존)
fn apply_footer_watermark(
    img: &mut RgbaImage,
    purchase_code: &str,
    page_num: i32,
    font: &ab_glyph::FontArc,
) {
    use ab_glyph::{Font, ScaleFont};

    let (width, height) = img.dimensions();

    let text = format!("{page_num}  |  {purchase_code}  |  Amazing Korean");
    let scale = ab_glyph::PxScale::from(18.0);
    let color = Rgba([153, 153, 153, 255]);

    // 폰트 메트릭으로 실제 텍스트 폭 계산
    let scaled_font = font.as_scaled(scale);
    let text_width: f32 = text.chars().fold(0.0, |acc, c| {
        acc + scaled_font.h_advance(font.glyph_id(c))
    });
    let text_width_px = text_width.ceil() as u32;

    // 풋터 영역 좌표 (구분선 y=height-122~height-119, 4px 두께 → 그 아래부터 클리어)
    let separator_bottom = height.saturating_sub(118); // 구분선 바로 아래
    let footer_bottom = height.saturating_sub(76);     // 풋터 텍스트 영역 하단
    let center_x = width / 2;
    let padding = 10u32; // 텍스트 좌우 여백
    let clear_x_start = center_x.saturating_sub(text_width_px / 2 + padding);
    let clear_x_end = (center_x + text_width_px / 2 + padding).min(width);

    // 구분선 아래, 텍스트 폭에 맞는 영역만 백색으로 클리어
    for y in separator_bottom..footer_bottom.min(height) {
        for x in clear_x_start..clear_x_end {
            img.put_pixel(x, y, Rgba([255, 255, 255, 255]));
        }
    }

    // 텍스트를 클리어 영역 내 세로 중앙에 배치
    let text_x = center_x.saturating_sub(text_width_px / 2);
    let text_y = separator_bottom + (footer_bottom.saturating_sub(separator_bottom)) / 2 - 9;

    draw_text_mut(img, color, text_x as i32, text_y as i32, scale, font, &text);
}

/// 마이크로 도트: 페이지 4곳 모서리 여백에 user_id를 1-2px 도트로 비트 인코딩
/// user_id (i64 = 64비트) → 4 모서리 × 16비트씩
/// 도트 색상: near-white (#FEFEFE) — 흰색 배경에서 거의 불가시
fn apply_microdot_watermark(img: &mut RgbaImage, user_id: i64) {
    let (width, height) = img.dimensions();
    let bits = user_id.to_be_bytes(); // 8바이트 = 64비트

    // 4개 모서리 시작 좌표 (가장자리에서 15-20px 안쪽)
    let corners: [(u32, u32); 4] = [
        (18, 18),                                   // 좌상
        (width.saturating_sub(85), 18),              // 우상
        (18, height.saturating_sub(25)),              // 좌하
        (width.saturating_sub(85), height.saturating_sub(25)), // 우하
    ];

    // near-white 도트 색상 (배경 흰색 대비 거의 불가시)
    let dot_color = Rgba([254, 254, 254, 255]);
    let dot_spacing = 4u32; // 도트 간격 4px

    for (corner_idx, &(start_x, start_y)) in corners.iter().enumerate() {
        // 각 코너에 16비트 배치 — y좌표 분산으로 단일 직선 패턴 방지
        for bit_offset in 0..16u32 {
            let global_bit = corner_idx * 16 + bit_offset as usize;
            let byte_idx = global_bit / 8;
            let bit_pos = 7 - (global_bit % 8);
            let bit_val = (bits[byte_idx] >> bit_pos) & 1;

            if bit_val == 1 {
                let x = start_x + bit_offset * dot_spacing;
                // y좌표를 비트 인덱스 기반으로 ±3px 분산 (짝수:+, 홀수:-)
                let y_offset = if bit_offset % 2 == 0 {
                    (bit_offset % 4) as i32
                } else {
                    -((bit_offset % 3) as i32 + 1)
                };
                let y = (start_y as i32 + y_offset).clamp(0, height as i32 - 1) as u32;

                // 범위 체크 후 1px 도트 배치
                if x < width && y < height {
                    img.put_pixel(x, y, dot_color);
                }
            }
        }
    }
}

/// LSB 스테가노그래피: purchase_code + watermark_id의 해시를 최하위 비트에 인코딩
fn apply_lsb_watermark(img: &mut RgbaImage, purchase_code: &str, watermark_id: &str) {
    let (width, height) = img.dimensions();
    let total_pixels = (width * height) as usize;

    if total_pixels < 128 {
        return; // 이미지가 너무 작으면 건너뜀
    }

    // 페이로드 생성: SHA-256 해시의 처음 8바이트 (64비트)
    let mut hasher = Sha256::new();
    hasher.update(purchase_code.as_bytes());
    hasher.update(b"|");
    hasher.update(watermark_id.as_bytes());
    let hash = hasher.finalize();
    let payload: [u8; 8] = hash[..8].try_into().unwrap();

    // 64비트 = 64개 픽셀의 R 채널 LSB에 인코딩
    // 각 비트마다 고유 시드 해시를 생성하여 픽셀 충돌 방지
    for bit_idx in 0..64usize {
        let byte_idx = bit_idx / 8;
        let bit_pos = 7 - (bit_idx % 8);
        let bit_val = (payload[byte_idx] >> bit_pos) & 1;

        // 비트별 고유 시드 생성 (purchase_code + bit_idx)
        let mut seed_hasher = Sha256::new();
        seed_hasher.update(b"lsb_seed|");
        seed_hasher.update(purchase_code.as_bytes());
        seed_hasher.update(b"|");
        seed_hasher.update(bit_idx.to_le_bytes());
        let seed = seed_hasher.finalize();

        let pixel_seed = u32::from_be_bytes([seed[0], seed[1], seed[2], seed[3]]);
        let pixel_idx = (pixel_seed as usize) % total_pixels;

        let x = (pixel_idx % width as usize) as u32;
        let y = (pixel_idx / width as usize) as u32;

        let pixel = img.get_pixel_mut(x, y);
        // R 채널의 LSB에 비트 인코딩
        pixel[0] = (pixel[0] & 0xFE) | bit_val;
    }
}

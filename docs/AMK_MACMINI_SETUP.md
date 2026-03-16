# Mac Mini M4 Pro 세팅 가이드

> **하드웨어**: M4 Pro 14코어 CPU, 20코어 GPU, 64GB RAM, 2TB SSD, 기가비트 이더넷
> **용도**: AI 인프라 허브 + iOS 개발 + 파이프라인 자동화
> **작성일**: 2026-03-10
> **관련 문서**: [AMK_PIPELINE.md §11](./AMK_PIPELINE.md#11-온디바이스-ai-전략-on-device-ai-strategy)

---

## 목차

- [Pre-arrival: 금요일 전 준비사항](#pre-arrival-금요일-전-준비사항)
- [Day 1: 하드웨어 + 기본 환경](#day-1-하드웨어--기본-환경)
- [Day 2: AI 인프라 (Priority 1)](#day-2-ai-인프라-priority-1)
- [Day 3: 개발 환경](#day-3-개발-환경)
- [Day 4: 원격 접속 + 자동화](#day-4-원격-접속--자동화)
- [Day 5: 검증 + 최적화](#day-5-검증--최적화)
- [참고: 포트 정리](#참고-포트-정리)
- [참고: 리소스 예산](#참고-리소스-예산)
- [트러블슈팅](#트러블슈팅)

---

## Pre-arrival: 금요일 전 준비사항

### 1. 구매 목록

| 품목 | 스펙 | 비고 |
|------|------|------|
| **KVM 스위치** | 2PC x 2모니터, HDMI 2.0, 4K@60Hz, USB 공유 | TESmart HKS0202A2U 또는 Avico 2x2 HDMI 2.0 |
| **USB-C to HDMI 어댑터** | HDMI 2.0 이상 지원 | Mac Mini HDMI 포트 1개 + USB-C 1개 → 듀얼 모니터용 |
| **이더넷 케이블** | Cat6 이상, 사무실 랜포트~Mac Mini 길이 | 기가비트 이더넷 활용 |
| **HDMI 케이블 x2** | HDMI 2.0, Mac Mini → KVM 연결용 | KVM에 포함될 수 있음, 확인 후 구매 |

### 2. 케이블 연결 다이어그램

```
[Windows PC]                              [Mac Mini M4 Pro]
  HDMI 1 ──→ ┌──────────┐ ←── HDMI (네이티브)
  HDMI 2 ──→ │ KVM      │ ←── USB-C → HDMI 어댑터
  USB    ──→ │ 스위치    │ ←── USB-A (또는 USB-C → USB-A)
             └──────────┘
              ↓↓       ↓
         [모니터1] [모니터2] [키보드+마우스]
```

### 3. API 키 발급

| 키 | 발급처 | 방법 | 비용 | 용도 |
|----|--------|------|------|------|
| **Anthropic API Key** | [console.anthropic.com](https://console.anthropic.com) | 회원가입 → API Keys → Create Key | 사용량 기반 (초기 $5 충전) | NanoClaw 구동 (필수) |
| **Gemini API Key** | [aistudio.google.com](https://aistudio.google.com) | Get API Key → Create | **무료** (신용카드 불필요) | 등급 1 자동 검증 + NadirClaw |
| **OpenAI Plus OAuth** | 기존 Plus 구독 | Codex CLI 연동 시 자동 인증 | **$0** (기존 구독) | 교차 검증 보조 (선택) |
| **Telegram Bot Token** | Telegram 앱 내 @BotFather | /newbot → 이름 설정 → 토큰 발급 | **무료** | 메신저 연동 |

> **비용 구조**: Anthropic API만 추가 비용 발생. Gemini API 무료 티어 (15회/분)는 등급 1 자동 검증에 충분. 등급 2~3 심층 검증은 기존 구독 웹 (Gemini AI Pro, ChatGPT Plus, Claude Max)에서 수동 처리.
>
> **검증 등급 체계**: 파일 경로/키워드/변경 규모에 따라 3단계로 자동 분류. 상세는 [AMK_PIPELINE.md §11.4.3](./AMK_PIPELINE.md) 참조.

### 4. Telegram Bot 생성 (상세)

```
1. Telegram에서 @BotFather 검색 → 대화 시작
2. /newbot 입력
3. 봇 이름 입력: "Amazing Korean Dev Bot" (표시 이름)
4. 봇 유저네임 입력: "amk_dev_bot" (고유, _bot으로 끝나야 함)
5. 발급된 토큰 저장: 123456789:ABCdefGHIjklMNO... 형식
6. /setdescription → 봇 설명 설정 (선택)
7. 봇에게 아무 메시지 전송 (채팅방 생성용)
```

---

## Day 1: 하드웨어 + 기본 환경

### 1.1 물리적 연결

```bash
# 순서
1. Mac Mini 전원 연결 (아직 켜지 않음)
2. 이더넷 케이블 연결 (사무실 랜포트 → Mac Mini 후면)
3. HDMI 케이블: Mac Mini HDMI → KVM 입력 1-A
4. USB-C to HDMI: Mac Mini USB-C → 어댑터 → KVM 입력 1-B
5. USB: Mac Mini USB-A → KVM USB 입력
6. Windows PC도 KVM에 연결 (기존 케이블 이동)
7. KVM 출력 → 모니터 1, 모니터 2, 키보드+마우스
8. Mac Mini 전원 ON
```

### 1.2 macOS 초기 설정

```
1. 언어: 한국어
2. 네트워크: 이더넷 자동 연결 확인
3. Apple ID 로그인
4. FileVault 암호화: 활성화 권장
5. 자동 업데이트: 활성화
6. 에너지 절약: "디스플레이가 꺼진 후에도 자동 절전 방지" → 체크 (24/7 운영)
```

### 1.3 시스템 설정

```bash
# 터미널 열기 (Cmd + Space → "Terminal")

# 1. Xcode Command Line Tools (Homebrew 전제 조건)
xcode-select --install

# 2. Homebrew 설치
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 3. 셸 설정에 Homebrew PATH 추가
echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zprofile
eval "$(/opt/homebrew/bin/brew shellenv)"

# 4. 기본 도구 설치
brew install git curl wget jq htop

# 5. SSH 활성화 (원격 접속용)
# 시스템 설정 → 일반 → 공유 → 원격 로그인 → 활성화
# 또는:
sudo systemsetup -setremotelogin on

# 6. 컴퓨터 이름 설정
sudo scutil --set ComputerName "amk-mac-mini"
sudo scutil --set HostName "amk-mac-mini"
sudo scutil --set LocalHostName "amk-mac-mini"

# 7. Git 설정
git config --global user.name "Your Name"
git config --global user.email "your@email.com"
```

### 1.4 에너지 설정 (24/7 운영용)

```bash
# 절전 모드 비활성화 (서버 역할이므로)
sudo pmset -a sleep 0          # 절전 안 함
sudo pmset -a disksleep 0      # 디스크 절전 안 함
sudo pmset -a displaysleep 30  # 디스플레이만 30분 후 꺼짐
sudo pmset -a womp 1           # Wake on LAN 활성화

# 전원 복구 시 자동 재시작
sudo pmset -a autorestart 1

# 설정 확인
pmset -g
```

---

## Day 2: AI 인프라 (Priority 1)

> **목표**: NanoClaw + NadirClaw + Ollama + BitNet.cpp 를 설치하고 Telegram으로 첫 메시지 주고받기

### 2.1 Ollama 설치 + 모델 다운로드

```bash
# Ollama 설치
brew install ollama

# Ollama 서비스 시작 (백그라운드)
brew services start ollama

# 모델 다운로드 (시간 소요, 네트워크 속도에 따라 30분~2시간)
# 간단한 요청용 (4.7GB)
ollama pull llama3.1:8b

# 코드 리뷰용 (19GB)
ollama pull qwen3:32b

# 고품질 교차 검증용 (42GB) — 나중에 다운로드해도 됨
# ollama pull qwen2.5:72b

# 설치 확인
ollama list
ollama run llama3.1:8b "Hello, are you working?"
```

> **다운로드 팁**: llama3.1:8b부터 먼저 받아서 다음 단계를 진행하면서 큰 모델은 백그라운드로 다운로드

### 2.2 NadirClaw 설치 + 설정

```bash
# Python 환경 (Homebrew)
brew install python@3.12

# NadirClaw 설치
pip3 install nadirclaw

# API 키 등록
nadirclaw auth add --provider google --key "YOUR_GEMINI_API_KEY"
nadirclaw auth add --provider anthropic --key "YOUR_ANTHROPIC_API_KEY"

# 라우팅 설정:
#   간단 → Gemini 2.5 Pro 무료 API (등급 1 자동 검증 겸용)
#   보통 → Ollama 로컬 (API 비용 $0)
#   복잡 → Claude API (유료, 최소화)
# 환경변수 설정 (~/.zshrc에 추가)
cat >> ~/.zshrc << 'EOF'

# NadirClaw 설정
export NADIRCLAW_SIMPLE_MODEL=gemini/gemini-2.5-pro
export NADIRCLAW_COMPLEX_MODEL=claude-sonnet-4-20250514
export NADIRCLAW_REASONING_MODEL=ollama/qwen3:32b
EOF

source ~/.zshrc

# NadirClaw 시작 + 테스트
nadirclaw serve --verbose &

# 테스트 (간단한 요청 → Ollama로 라우팅되는지 확인)
curl http://localhost:8856/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model": "auto", "messages": [{"role": "user", "content": "2+2는?"}]}'

# 테스트 (복잡한 요청 → Claude로 라우팅되는지 확인)
curl http://localhost:8856/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model": "auto", "messages": [{"role": "user", "content": "Rust에서 async trait을 구현할 때 lifetime 문제를 해결하는 패턴을 설명해줘"}]}'
```

### 2.3 NanoClaw 설치 + Telegram 연동

```bash
# Node.js 설치
brew install node@20

# NanoClaw 클론
cd ~/
git clone https://github.com/qwibitai/nanoclaw.git
cd nanoclaw
npm install

# 환경 설정
cat > .env << EOF
ANTHROPIC_API_KEY=YOUR_ANTHROPIC_API_KEY
TELEGRAM_BOT_TOKEN=YOUR_TELEGRAM_BOT_TOKEN
EOF

# Claude Code로 셋업 (AI 네이티브 설정)
# NanoClaw는 Claude가 설정을 가이드하는 방식
claude

# Claude Code 안에서:
# /setup 실행 → 의존성 설치, 컨테이너 구성, 메신저 연결

# 또는 수동으로 Telegram 연동:
# /add-telegram 실행

# NanoClaw 시작
npm start
```

### 2.4 NanoClaw 그룹 설정

```bash
# Telegram에서 봇에게 메시지 보내기로 그룹 생성
# 그룹별 CLAUDE.md 메모리 설정

# 파이프라인 알림 그룹
mkdir -p ~/nanoclaw/groups/pipeline-alerts
cat > ~/nanoclaw/groups/pipeline-alerts/CLAUDE.md << 'EOF'
# Pipeline Alerts

이 그룹은 Amazing Korean 프로젝트의 파이프라인 알림 전용입니다.

## 프로젝트 정보
- 백엔드: Rust + Axum + SQLx + PostgreSQL
- 프론트엔드: React + Vite + TypeScript
- 배포: AWS EC2 (백엔드) + Cloudflare Pages (프론트엔드)
- 도메인: api.amazingkorean.net

## 역할
- GitHub PR/빌드 알림 수신 및 요약
- 서버 상태 모니터링 결과 전달
- 품질 게이트 결과 보고
EOF

# 검증 그룹 (등급 체계 적용)
mkdir -p ~/nanoclaw/groups/verification
cat > ~/nanoclaw/groups/verification/CLAUDE.md << 'EOF'
# Verification Agent

Amazing Korean 프로젝트의 검증 전용 그룹입니다.
검증 등급 체계 (AMK_PIPELINE.md §11.4.3)를 따릅니다.

## 검증 등급 판정 규칙

### 등급 3 (전문가 검증) — 최우선 판정
해당 파일/키워드가 포함되면 무조건 등급 3:
- 파일: src/crypto/*, src/api/payment/*, src/api/auth/service.rs, migrations/*.sql
- 키워드: 암호화, 보안, 결제, 인증, 마이그레이션, 프로덕션

### 등급 2 (심층 검증)
- 파일: **/service.rs, **/router.rs, 기획서, 스키마 문서
- 키워드: 새 기능, 설계, 스키마 변경, 아키텍처

### 등급 1 (자동 검증) — 기본값
- 위에 해당하지 않는 모든 것

## 등급별 처리 방식

### 등급 1
Gemini 2.5 Pro API (무료)로 자동 검증 후 결과 전달.
형식: "✅ 등급1 자동 검증 완료: [요약]"

### 등급 2
문서를 md로 정리 + 검증 프롬프트 생성 후 전달.
형식:
```
⚠️ 등급2 심층 검증 필요
📄 대상: [파일명/섹션]
아래 프롬프트를 Gemini 3.1 Pro 웹에 붙여넣으세요:
─── 복사 시작 ───
[대상 문서 + 검증 관점 프롬프트]
─── 복사 끝 ───
```

### 등급 3
3개 모델용 검증 프롬프트 각각 생성 후 전달.
형식:
```
🔴 등급3 전문가 검증 필요
📄 대상: [파일명/섹션]
최소 2개 모델에서 교차 검증하세요:
1️⃣ Gemini 3.1 Pro 용: [보안 관점 프롬프트]
2️⃣ ChatGPT 용: [로직 완결성 프롬프트]
3️⃣ Claude 웹 용 (선택): [타입 안전성 프롬프트]
→ 2개 이상 합의 시 승인
→ 불일치 시 추가 검토 필요
```

## 판단 원칙
- 애매하면 상위 등급으로 올린다
- 등급 3 대상 파일 경로는 변경 규모와 무관하게 항상 등급 3
EOF

# 일반 작업 그룹
mkdir -p ~/nanoclaw/groups/general
cat > ~/nanoclaw/groups/general/CLAUDE.md << 'EOF'
# General Assistant

Amazing Korean 프로젝트의 일반 질문 및 작업 지시용 그룹입니다.

## 사용 가능 도구
- NadirClaw 경유 LLM 호출 (localhost:8856)
- Ollama 로컬 모델
- 파일 시스템 (마운트된 디렉토리만)
EOF
```

### 2.5 BitNet.cpp 설치

```bash
# Conda 설치 (Miniforge 권장, Apple Silicon 최적화)
brew install miniforge
conda init zsh
source ~/.zshrc

# BitNet 환경 생성
conda create -n bitnet-cpp python=3.9 -y
conda activate bitnet-cpp

# BitNet.cpp 클론 및 설치
cd ~/
git clone https://github.com/microsoft/BitNet.git
cd BitNet
pip install -r requirements.txt

# 모델 다운로드 + 테스트
python run_inference.py \
  -m models/BitNet-b1.58-2B-4T/ggml-model-i2_s.gguf \
  -p "You are a helpful Korean language tutor" \
  -cnv

# 테스트 후 conda 비활성화
conda deactivate
```

### 2.6 24/7 서비스 등록 (launchd)

```bash
# Ollama는 brew services로 이미 등록됨

# NadirClaw 자동 시작 설정
cat > ~/Library/LaunchAgents/com.nadirclaw.server.plist << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.nadirclaw.server</string>
    <key>ProgramArguments</key>
    <array>
        <string>/opt/homebrew/bin/nadirclaw</string>
        <string>serve</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/tmp/nadirclaw.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/nadirclaw.error.log</string>
    <key>EnvironmentVariables</key>
    <dict>
        <key>NADIRCLAW_SIMPLE_MODEL</key>
        <string>ollama/llama3.1:8b</string>
        <key>NADIRCLAW_COMPLEX_MODEL</key>
        <string>claude-sonnet-4-20250514</string>
    </dict>
</dict>
</plist>
EOF

launchctl load ~/Library/LaunchAgents/com.nadirclaw.server.plist

# NanoClaw 자동 시작 설정
cat > ~/Library/LaunchAgents/com.nanoclaw.server.plist << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.nanoclaw.server</string>
    <key>ProgramArguments</key>
    <array>
        <string>/opt/homebrew/bin/node</string>
        <string>/Users/YOUR_USERNAME/nanoclaw/index.js</string>
    </array>
    <key>WorkingDirectory</key>
    <string>/Users/YOUR_USERNAME/nanoclaw</string>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/tmp/nanoclaw.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/nanoclaw.error.log</string>
</dict>
</plist>
EOF

launchctl load ~/Library/LaunchAgents/com.nanoclaw.server.plist

# 서비스 상태 확인
launchctl list | grep -E "nadirclaw|nanoclaw|ollama"
```

### 2.7 Day 2 검증 체크리스트

```bash
# ✅ Ollama 작동 확인
ollama run llama3.1:8b "한국어로 인사해줘"

# ✅ NadirClaw 라우팅 확인
curl -s http://localhost:8856/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model": "auto", "messages": [{"role": "user", "content": "hi"}]}' | jq .model

# ✅ Telegram 봇 응답 확인
# Telegram에서 봇에게 "안녕" 전송 → 응답 확인

# ✅ BitNet 실행 확인
conda activate bitnet-cpp && python ~/BitNet/run_inference.py \
  -m ~/BitNet/models/BitNet-b1.58-2B-4T/ggml-model-i2_s.gguf \
  -p "What is 2+2?" -n 50
```

---

## Day 3: 개발 환경

### 3.1 Xcode 설치

```bash
# App Store에서 Xcode 설치 (약 30GB, 시간 소요)
# 설치 중 다른 작업 병행 가능

# 설치 후 라이선스 동의
sudo xcodebuild -license accept

# iOS 시뮬레이터 런타임 다운로드
xcodebuild -downloadPlatform iOS
```

### 3.2 Rust 툴체인

```bash
# Rust 설치
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 프로젝트에 필요한 타겟 추가 (iOS용, 나중에 필요 시)
# rustup target add aarch64-apple-ios aarch64-apple-ios-sim

# 확인
rustc --version
cargo --version
```

### 3.3 Node.js + 프론트엔드 도구

```bash
# Node.js는 Day 2에서 이미 설치됨 (brew install node@20)
node --version
npm --version
```

### 3.4 Docker (OrbStack 권장)

```bash
# OrbStack 설치 (Docker Desktop보다 Mac에서 가볍고 빠름)
brew install orbstack

# 또는 Docker Desktop
# brew install --cask docker

# Docker 확인
docker --version
docker run hello-world

# 개발용 PostgreSQL + Redis (프로젝트 로컬 테스트용)
docker run -d \
  --name amk-postgres \
  -e POSTGRES_USER=amk \
  -e POSTGRES_PASSWORD=devpassword \
  -e POSTGRES_DB=amazing_korean \
  -p 5432:5432 \
  postgres:16

docker run -d \
  --name amk-redis \
  -p 6379:6379 \
  redis:7-alpine
```

### 3.5 프로젝트 클론 및 빌드 확인

```bash
# 프로젝트 클론
cd ~/dev
git clone https://github.com/YOUR_REPO/amazing-korean-api.git
cd amazing-korean-api

# 백엔드 빌드 확인
cargo check

# 프론트엔드 빌드 확인
cd frontend && npm install && npm run build
```

### 3.6 Python 과학 도구 (발음 교정 AI용)

```bash
# 발음 교정 프로토타이핑 환경 (§11.9 Phase P-1)
conda create -n pronunciation-ai python=3.11 -y
conda activate pronunciation-ai

# 피치 분석 도구
pip install librosa crepe numpy scipy

# Whisper (음성 인식)
pip install openai-whisper

# 오디오 처리
brew install ffmpeg
pip install pydub soundfile

conda deactivate
```

---

## Day 4: 원격 접속 + 자동화

### 4.1 Tailscale (VPN)

```bash
# Tailscale 설치
brew install --cask tailscale

# 또는 App Store에서 Tailscale 설치

# Tailscale 시작 → 로그인 (Google/GitHub 계정)
# 메뉴바 아이콘 → Log in

# Windows PC에도 Tailscale 설치
# → https://tailscale.com/download/windows

# 양쪽 모두 같은 계정으로 로그인하면 자동 연결

# Mac Mini의 Tailscale IP 확인
tailscale ip -4
# 예: 100.x.y.z

# Windows에서 Mac Mini SSH 접속 테스트
# (Windows PowerShell)
# ssh your_username@100.x.y.z
```

### 4.2 VS Code Remote SSH (Windows에서)

```
Windows PC에서:
1. VS Code 설치 (이미 있으면 스킵)
2. "Remote - SSH" 확장 설치
3. Cmd+Shift+P → "Remote-SSH: Add New SSH Host"
4. ssh your_username@100.x.y.z (Tailscale IP)
5. 연결 후 Mac Mini의 프로젝트 폴더 열기
```

### 4.3 GitHub Actions Self-hosted Runner

```bash
# GitHub 리포지토리 → Settings → Actions → Runners → New self-hosted runner

# macOS ARM64 선택 후 안내에 따라 설치
mkdir ~/actions-runner && cd ~/actions-runner
curl -o actions-runner-osx-arm64-2.x.x.tar.gz -L https://github.com/actions/runner/releases/download/v2.x.x/actions-runner-osx-arm64-2.x.x.tar.gz
tar xzf ./actions-runner-osx-arm64-2.x.x.tar.gz

# 구성
./config.sh --url https://github.com/YOUR_REPO --token YOUR_TOKEN

# 서비스로 설치 (24/7)
./svc.sh install
./svc.sh start
./svc.sh status
```

### 4.4 모니터링 (선택적)

```bash
# Uptime Kuma (경량 모니터링 대시보드)
docker run -d \
  --name uptime-kuma \
  -p 3001:3001 \
  -v uptime-kuma:/app/data \
  --restart always \
  louislam/uptime-kuma:1

# 브라우저에서 http://localhost:3001 접속
# 모니터링 대상 추가:
# - https://api.amazingkorean.net/health (백엔드)
# - https://amazingkorean.net (프론트엔드)
# - http://localhost:11434 (Ollama)
# - http://localhost:8856 (NadirClaw)
# 알림 채널: Telegram Bot 연결
```

---

## Day 5: 검증 + 최적화

### 5.1 전체 시스템 검증

```bash
echo "=== 시스템 상태 확인 ==="

echo "--- 서비스 상태 ---"
launchctl list | grep -E "nadirclaw|nanoclaw|ollama"

echo "--- Ollama 모델 ---"
ollama list

echo "--- NadirClaw ---"
curl -s http://localhost:8856/v1/models | jq .

echo "--- Docker 컨테이너 ---"
docker ps

echo "--- 디스크 사용량 ---"
df -h /

echo "--- 메모리 사용량 ---"
memory_pressure

echo "--- Tailscale ---"
tailscale status
```

### 5.2 통합 테스트

```bash
# 1. Telegram → NanoClaw → NadirClaw → Ollama (간단 요청)
#    Telegram에서 "오늘 날씨 어때?" 전송
#    → NanoClaw가 NadirClaw 경유 → Ollama로 라우팅 → 응답

# 2. Telegram → NanoClaw → NadirClaw → Claude (복잡 요청)
#    Telegram에서 "Rust에서 async trait 패턴 설명해줘" 전송
#    → NadirClaw가 복잡도 감지 → Claude API로 라우팅 → 응답

# 3. Tailscale + VS Code Remote
#    Windows PC에서 VS Code → Mac Mini 원격 접속
#    → amazing-korean-api 프로젝트 열기 → cargo check 실행

# 4. 빌드 확인
cd ~/dev/amazing-korean-api
cargo check           # 백엔드
cd frontend && npm run build  # 프론트엔드
```

### 5.3 launchd 서비스 자동 복구 확인

```bash
# Ollama 강제 종료 → 자동 재시작 확인
pkill ollama
sleep 5
curl -s http://localhost:11434/api/tags | jq .  # 재시작 확인

# NadirClaw 강제 종료 → 자동 재시작 확인
pkill nadirclaw
sleep 5
curl -s http://localhost:8856/v1/models | jq .  # 재시작 확인
```

---

## 참고: 포트 정리

| 포트 | 서비스 | 용도 |
|------|--------|------|
| 11434 | Ollama | 로컬 LLM API |
| 8856 | NadirClaw | LLM 라우터 프록시 |
| 5432 | PostgreSQL | 개발용 DB |
| 6379 | Redis | 개발용 캐시 |
| 3001 | Uptime Kuma | 모니터링 대시보드 |

---

## 참고: 리소스 예산

### Layer 1: 항시 가동 (24/7)

| 서비스 | CPU | RAM | 디스크 |
|--------|-----|-----|--------|
| EXAONE 4.0-1.2B (문지기) | 1코어 | ~1GB | ~1GB |
| NadirClaw | ~0 | ~0.1GB | ~50MB |
| NanoClaw | ~0 | ~0.5GB | ~100MB |
| Uptime Kuma | ~0 | ~0.1GB | ~100MB |
| **소계** | **1코어** | **~1.7GB** | - |

### Layer 2: 온디맨드 (사용 시)

| 서비스 | CPU | RAM | 디스크 |
|--------|-----|-----|--------|
| Qwen3-Coder-30B-A3B (코드 리뷰, MoE) | 4-6코어 | ~18GB | ~18GB |
| EXAONE 4.0-32B (한국어) | 4-6코어 | ~18GB | ~18GB |
| AX 3.1 Lite 7B (빠른 한국어) | 2-3코어 | ~4.5GB | ~4.5GB |
| NLLB-200-3.3B (22개 언어 번역) | 1-2코어 | ~3.5GB | ~3.5GB |
| Xcode + 시뮬레이터 | 4-6코어 | ~16GB | ~50GB |
| Docker (PG+Redis) | 1-2코어 | ~1GB | ~2GB |
| cargo build | 4-8코어 | ~4-8GB | ~2GB |

### 최악의 동시 사용

```
Layer 1 (항시)           ~1.7GB
+ Qwen3-Coder-30B 추론  ~18GB
+ Xcode 빌드            ~16GB
+ Docker                 ~1GB
─────────────────────────────
합계                    ~37GB / 64GB (여유: 27GB)
```

---

## 참고: 도구별 역할 요약

> 상세: [`AMK_PIPELINE.md §11.4.4`](./AMK_PIPELINE.md#1144-도구별-역할-정의-tool-role-map)

| 도구 | 별명 | 가동 방식 | 핵심 역할 |
|------|------|----------|----------|
| **EXAONE 4.0-1.2B** | 항상 깨어있는 문지기 | 24/7 (~1GB) | pre-commit hook, 등급 분류, PR 요약 (한영 네이티브) |
| **NanoClaw** | 알림/자동화 허브 | 24/7 (~1GB) | Telegram 연동, 검증 워크플로우 자동화 |
| **NadirClaw** | 지능형 라우터 | 24/7 (~0.5GB) | ~10ms 복잡도 분류 → 최적 모델 라우팅 |
| **Ollama** | 필요할 때 부르는 전문가 | 온디맨드 | 코드 리뷰, 한국어 콘텐츠, 번역, 오프라인 안전망 |
| **Claude Code** | 핵심 실행자 | 온디맨드 | 코드 구현/수정/디버깅 (VS Code Remote SSH) |

**주요 Ollama 모델:**

| 모델 | 크기 (Q4) | 용도 |
|------|----------|------|
| Qwen3-Coder-30B-A3B | ~18GB | 코드 리뷰 (Rust/TS/SQL), LiveCodeBench 1위 |
| EXAONE 4.0-32B | ~18GB | 한국어 콘텐츠 생성, 교차 검증 |
| AX 3.1 Lite 7B | ~4.5GB | 빠른 한국어 응답 |
| NLLB-200-3.3B | ~3.5GB | 22개 언어 번역 |
| ENERZAi Korean Whisper | ~0.5GB | 한국어 STT/발음 평가 |

---

## 트러블슈팅

### KVM 전환 시 Mac Mini 화면이 안 나올 때

```
1. KVM 전환 후 5초 대기 (EDID 핸드셰이크 시간)
2. 안 되면: Mac Mini 키보드에서 아무 키 입력 (디스플레이 깨우기)
3. 계속 안 되면: KVM EDID 에뮬레이션 설정 확인
4. 최후 수단: USB-C to HDMI 어댑터를 EDID 에뮬레이터 지원 제품으로 교체
```

### Ollama 모델 로드가 느릴 때

```bash
# 모델이 RAM에서 언로드된 후 재로드 시 느림 (정상)
# 자주 쓰는 모델을 메모리에 유지:
OLLAMA_KEEP_ALIVE=24h ollama serve

# 또는 환경변수로 영구 설정
echo 'export OLLAMA_KEEP_ALIVE=24h' >> ~/.zshrc
```

### NanoClaw Telegram 연결 끊김

```bash
# 로그 확인
tail -f /tmp/nanoclaw.log
tail -f /tmp/nanoclaw.error.log

# 서비스 재시작
launchctl unload ~/Library/LaunchAgents/com.nanoclaw.server.plist
launchctl load ~/Library/LaunchAgents/com.nanoclaw.server.plist
```

### Mac Mini가 절전에서 안 깨어날 때

```bash
# Wake on LAN 확인
sudo pmset -g | grep womp
# womp = 1 이어야 함

# 절전 완전 비활성화 재확인
sudo pmset -a sleep 0
```

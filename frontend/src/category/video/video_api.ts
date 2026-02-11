import { request } from "@/api/client";
import type {
  VideoDetail,
  VideoListReq,
  VideoListRes,
  VideoProgressUpdateReq,
  VideoProgressRes,
} from "@/category/video/types";

export const getVideoList = (params: VideoListReq = {}, lang?: string) => {
  return request<VideoListRes>("/videos", {
    params: { ...params, lang },
  });
};

export const getVideoDetail = (id: number, lang?: string) => {
  return request<VideoDetail>(`/videos/${id}`, {
    params: lang ? { lang } : undefined,
  });
};

export const getVideoProgress = (videoId: number) => {
  return request<VideoProgressRes>(`/videos/${videoId}/progress`);
};

export const updateVideoProgress = (videoId: number, data: VideoProgressUpdateReq) => {
  return request<void>(`/videos/${videoId}/progress`, {
    method: "POST",
    data,
  });
};

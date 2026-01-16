import { request } from "@/api/client";
import type {
  VideoDetail,
  VideoListReq,
  VideoListRes,
  VideoProgressUpdateReq,
  VideoProgressRes,
} from "@/category/video/types";

export const getVideoList = (params: VideoListReq = {}) => {
  return request<VideoListRes>("/videos", {
    params,
  });
};

export const getVideoDetail = (id: number) => {
  return request<VideoDetail>(`/videos/${id}`);
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

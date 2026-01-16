import { request } from "@/api/client";
import type {
  VideoDetail,
  VideoListReq,
  VideoListRes,
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

export const getVideoProgress = (videoId: string) => {
  return request<VideoProgressRes>(`/videos/${videoId}/progress`);
};

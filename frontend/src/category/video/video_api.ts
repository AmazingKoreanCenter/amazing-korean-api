import { request } from "@/api/client";
import type {
  VideoDetail,
  VideoListReq,
  VideoListRes,
} from "@/category/video/types";

export const getVideoList = (params: VideoListReq = {}) => {
  return request<VideoListRes>("/videos", {
    params,
  });
};

export const getVideoDetail = (id: number) => {
  return request<VideoDetail>(`/videos/${id}`);
};

import { request } from "@/api/client";
import type { VideoListReq, VideoListRes } from "@/category/video/types";

export const getVideoList = (params: VideoListReq = {}) => {
  return request<VideoListRes>("/videos", {
    params,
  });
};

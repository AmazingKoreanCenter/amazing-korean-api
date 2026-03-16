import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";

import {
  getAdminTextbookOrders,
  getAdminTextbookOrder,
  updateAdminTextbookOrderStatus,
  updateAdminTextbookOrderTracking,
  deleteAdminTextbookOrder,
} from "../../admin_api";
import type { AdminTextbookListReq, AdminUpdateStatusReq, AdminUpdateTrackingReq } from "../types";

// =============================================================================
// Query Keys
// =============================================================================

export const adminTextbookKeys = {
  orders: ["admin", "textbook", "orders"] as const,
  orderList: (params: AdminTextbookListReq) =>
    [...adminTextbookKeys.orders, "list", params] as const,
  orderDetail: (id: number) =>
    [...adminTextbookKeys.orders, "detail", id] as const,
};

// =============================================================================
// Queries
// =============================================================================

export const useAdminTextbookOrders = (params: AdminTextbookListReq) => {
  return useQuery({
    queryKey: adminTextbookKeys.orderList(params),
    queryFn: () => getAdminTextbookOrders(params),
  });
};

export const useAdminTextbookOrderDetail = (id: number) => {
  return useQuery({
    queryKey: adminTextbookKeys.orderDetail(id),
    queryFn: () => getAdminTextbookOrder(id),
    enabled: id > 0,
  });
};

// =============================================================================
// Mutations
// =============================================================================

export const useAdminUpdateTextbookStatus = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: number; data: AdminUpdateStatusReq }) =>
      updateAdminTextbookOrderStatus(id, data),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({
        queryKey: adminTextbookKeys.orders,
      });
      queryClient.invalidateQueries({
        queryKey: adminTextbookKeys.orderDetail(id),
      });
    },
  });
};

export const useAdminUpdateTextbookTracking = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: number; data: AdminUpdateTrackingReq }) =>
      updateAdminTextbookOrderTracking(id, data),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({
        queryKey: adminTextbookKeys.orders,
      });
      queryClient.invalidateQueries({
        queryKey: adminTextbookKeys.orderDetail(id),
      });
    },
  });
};

export const useAdminDeleteTextbookOrder = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: number) => deleteAdminTextbookOrder(id),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: adminTextbookKeys.orders,
      });
    },
  });
};

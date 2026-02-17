import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";

import {
  getAdminSubscriptions,
  getAdminSubscription,
  adminCancelSubscription,
  getAdminTransactions,
  createAdminGrant,
  getAdminGrants,
  revokeAdminGrant,
} from "../../admin_api";
import type {
  AdminSubListReq,
  AdminCancelSubReq,
  AdminTxnListReq,
  AdminGrantReq,
  AdminGrantListReq,
} from "../types";

// ==========================================
// Query Keys
// ==========================================

export const adminPaymentKeys = {
  subscriptions: ["admin", "payment", "subscriptions"] as const,
  subscriptionList: (params: AdminSubListReq) =>
    [...adminPaymentKeys.subscriptions, "list", params] as const,
  subscriptionDetail: (id: number) =>
    [...adminPaymentKeys.subscriptions, "detail", id] as const,
  transactions: ["admin", "payment", "transactions"] as const,
  transactionList: (params: AdminTxnListReq) =>
    [...adminPaymentKeys.transactions, "list", params] as const,
  grants: ["admin", "payment", "grants"] as const,
  grantList: (params: AdminGrantListReq) =>
    [...adminPaymentKeys.grants, "list", params] as const,
};

// ==========================================
// Subscription Queries
// ==========================================

export const useAdminSubscriptions = (params: AdminSubListReq) => {
  return useQuery({
    queryKey: adminPaymentKeys.subscriptionList(params),
    queryFn: () => getAdminSubscriptions(params),
  });
};

export const useAdminSubscriptionDetail = (id: number) => {
  return useQuery({
    queryKey: adminPaymentKeys.subscriptionDetail(id),
    queryFn: () => getAdminSubscription(id),
    enabled: id > 0,
  });
};

// ==========================================
// Subscription Mutations
// ==========================================

export const useAdminCancelSubscription = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: number; data: AdminCancelSubReq }) =>
      adminCancelSubscription(id, data),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({
        queryKey: adminPaymentKeys.subscriptions,
      });
      queryClient.invalidateQueries({
        queryKey: adminPaymentKeys.subscriptionDetail(id),
      });
    },
  });
};

// ==========================================
// Transaction Queries
// ==========================================

export const useAdminTransactions = (params: AdminTxnListReq) => {
  return useQuery({
    queryKey: adminPaymentKeys.transactionList(params),
    queryFn: () => getAdminTransactions(params),
  });
};

// ==========================================
// Grant Queries & Mutations
// ==========================================

export const useAdminGrants = (params: AdminGrantListReq) => {
  return useQuery({
    queryKey: adminPaymentKeys.grantList(params),
    queryFn: () => getAdminGrants(params),
  });
};

export const useCreateAdminGrant = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: AdminGrantReq) => createAdminGrant(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminPaymentKeys.grants });
    },
  });
};

export const useRevokeAdminGrant = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (userId: number) => revokeAdminGrant(userId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminPaymentKeys.grants });
    },
  });
};

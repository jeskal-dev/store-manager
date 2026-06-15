import { useQuery } from "@tanstack/react-query";
import { getDashboardData } from "@/client/commands";
import type { DashboardData } from "@/client/types";

type UseDashboardOptions = {
  days?: number;
  storeId?: string;
};

export function useDashboard({ days = 30, storeId }: UseDashboardOptions = {}) {
  return useQuery<DashboardData>({
    queryKey: ["dashboard", { days, storeId }],
    queryFn: () =>
      getDashboardData({
        input: {
          days,
          store_id: storeId,
        },
      }),
  });
}

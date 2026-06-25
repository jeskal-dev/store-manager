import { keepPreviousData, queryOptions } from "@tanstack/react-query";
import { getDashboardData } from "@/client/commands";
import type { DashboardData } from "@/client/types";

type UseDashboardOptions = {
  days?: number;
  storeId?: string;
};

export const dashboardQuery = ({ days = 30, storeId }: UseDashboardOptions) =>
  queryOptions<DashboardData>({
    queryKey: ["dashboard", { days, storeId }],
    queryFn: () =>
      getDashboardData({
        input: {
          days,
          storeId: storeId,
        },
      }),
    placeholderData: keepPreviousData,
  });

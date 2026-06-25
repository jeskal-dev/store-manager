import { DashboardPage } from "@/components/dashboard/dashboard.page";
import { dashboardQuery } from "@/hooks/dashboard/use-dashboard";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_main/")({
  component: DashboardPage,
  loader: async ({ context }) => {
    context.queryClient.prefetchQuery(dashboardQuery({ days: 30 }));
  },
});

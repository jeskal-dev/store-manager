import { type QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { TooltipProvider } from "../ui/tooltip";

interface ProviderWrapperProps {
  children: React.ReactNode;
  queryClient: QueryClient;
}

export function ProviderWrapper({
  children,
  queryClient,
}: ProviderWrapperProps) {
  return (
    <QueryClientProvider client={queryClient}>
      <TooltipProvider>{children}</TooltipProvider>
    </QueryClientProvider>
  );
}

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { cn } from "@/lib/utils";

type MetricCardProps = {
  title: string;
  value: string;
  subtitle?: string;
  alert?: boolean;
};

export function MetricCard({ title, value, subtitle, alert }: MetricCardProps) {
  return (
    <Card
      size="sm"
      className={cn(
        "border-t-2 transition-all hover:bg-muted/50",
        alert
          ? "border-t-destructive/30 ring-destructive/30"
          : "border-t-accent/40"
      )}
    >
      <CardHeader className="pb-0">
        <CardTitle className="font-heading text-xs font-semibold tracking-normal text-muted-foreground">
          {title}
        </CardTitle>
      </CardHeader>
      <CardContent>
        <p
          className={cn(
            "font-sans text-2xl font-bold tabular-nums tracking-tighter",
            alert && "text-destructive"
          )}
        >
          {value}
        </p>
        {subtitle && (
          <p className="mt-0.5 text-xs text-muted-foreground">{subtitle}</p>
        )}
      </CardContent>
    </Card>
  );
}

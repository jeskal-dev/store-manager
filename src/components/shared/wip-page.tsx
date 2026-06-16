import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";

type Section =
  | { type: "title" }
  | { type: "filter-bar" }
  | { type: "cards"; count: number }
  | { type: "table"; rows: number; cols: number }
  | { type: "form"; fields: number };

type WipPageProps = {
  title: string;
  description: string;
  sections: Section[];
};

export function WipPage({ title, description, sections }: WipPageProps) {
  return (
    <div className="space-y-6">
      <div className="flex items-start justify-between">
        <div className="space-y-1">
          <h1 className="text-sm font-medium">{title}</h1>
          <p className="text-xs text-muted-foreground">{description}</p>
        </div>
        <Badge variant="outline" className="text-[10px]">
          En desarrollo
        </Badge>
      </div>

      <Separator />

      {sections.map((section, i) => (
        <div key={i}>{renderSection(section)}</div>
      ))}

      <div className="flex items-center justify-center gap-2 rounded-none border border-dashed border-border/50 bg-muted/30 px-4 py-8">
        <p className="text-xs text-muted-foreground">
          Esta sección se encuentra en desarrollo y estará disponible
          próximamente
        </p>
      </div>
    </div>
  );
}

function renderSection(section: Section) {
  switch (section.type) {
    case "title":
      return <Skeleton className="h-5 w-48" />;

    case "filter-bar":
      return (
        <div className="flex items-center gap-2">
          <Skeleton className="h-7 w-32" />
          <Skeleton className="h-7 w-28" />
          <div className="flex-1" />
          <Skeleton className="h-7 w-24" />
        </div>
      );

    case "cards":
      return (
        <div className="grid grid-cols-2 gap-3 md:grid-cols-4">
          {Array.from({ length: section.count }).map((_, i) => (
            <Skeleton key={i} className="h-24 rounded-none" />
          ))}
        </div>
      );

    case "table":
      return (
        <div className="space-y-1">
          <Skeleton className="h-8 w-full rounded-none" />
          {Array.from({ length: section.rows }).map((_, i) => (
            <Skeleton
              key={i}
              className="h-7 w-full rounded-none"
              style={{ opacity: 1 - i * 0.12 }}
            />
          ))}
        </div>
      );

    case "form":
      return (
        <div className="space-y-3">
          {Array.from({ length: section.fields }).map((_, i) => (
            <div key={i} className="space-y-1">
              <Skeleton className="h-3 w-16" />
              <Skeleton className="h-8 w-full rounded-none" />
            </div>
          ))}
          <Skeleton className="h-8 w-28 rounded-none" />
        </div>
      );
  }
}

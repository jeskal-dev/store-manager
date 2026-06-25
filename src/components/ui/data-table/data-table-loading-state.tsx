import { IconLoader2 } from "@tabler/icons-react";
import { TableCell, TableRow } from "../table";
import { Alert, AlertDescription, AlertTitle } from "../alert";
import type { DataTableStateProps } from "@/types/data-table";

export function DataTableLoadingState({
  colSpan,
  message = "Cargando datos...",
}: DataTableStateProps) {
  return (
    <TableRow>
      <TableCell colSpan={colSpan} className="h-24 text-center">
        <Alert className="max-w-md mx-auto shadow-sm border-primary/20">
          <IconLoader2 className="h-5 w-5 animate-spin text-primary" />
          <AlertTitle className="font-medium text-primary">Cargando</AlertTitle>
          <AlertDescription className="text-muted-foreground">
            {message}
          </AlertDescription>
        </Alert>
      </TableCell>
    </TableRow>
  );
}

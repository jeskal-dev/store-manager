import { IconInbox } from "@tabler/icons-react";
import { TableCell, TableRow } from "../table";
import { Alert, AlertDescription, AlertTitle } from "../alert";
import type { DataTableStateProps } from "@/types/data-table";

export function DataTableEmptyState({
  colSpan,
  message = "Sin resultados.",
}: DataTableStateProps) {
  return (
    <TableRow>
      <TableCell colSpan={colSpan} className="h-24 text-center">
        <Alert className="max-w-md mx-auto shadow-sm border-muted-foreground/20">
          <IconInbox className="h-5 w-5 text-muted-foreground" />
          <AlertTitle className="font-medium text-muted-foreground">
            No hay datos
          </AlertTitle>
          <AlertDescription className="text-muted-foreground">
            {message}
          </AlertDescription>
        </Alert>
      </TableCell>
    </TableRow>
  );
}

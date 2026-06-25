import { IconAlertCircle } from "@tabler/icons-react";
import { TableCell, TableRow } from "../table";
import { Button } from "../button";
import { Alert, AlertDescription, AlertTitle } from "../alert";
import type { DataTableStateProps } from "@/types/data-table";
import { formatApiError } from "@/lib/format-errors";

interface DataTableErrorStateProps extends DataTableStateProps {
  error?: Error | null;
  onRetry?: () => void;
}

export function DataTableErrorState({
  colSpan,
  message = "Error al cargar los datos.",
  error,
  onRetry,
}: DataTableErrorStateProps) {
  return (
    <TableRow>
      <TableCell colSpan={colSpan} className="h-24 text-center">
        <Alert variant="destructive" className="max-w-md mx-auto shadow-sm">
          <IconAlertCircle className="h-5 w-5" />
          <AlertTitle className="font-semibold">Error</AlertTitle>
          <AlertDescription className="flex flex-col gap-3">
            <span>{formatApiError(error, message)}</span>
            {onRetry && (
              <Button
                onClick={onRetry}
                variant="outline"
                size="sm"
                className="w-fit border-destructive/30 text-destructive hover:bg-destructive/10 hover:text-destructive"
              >
                Intentar nuevamente
              </Button>
            )}
          </AlertDescription>
        </Alert>
      </TableCell>
    </TableRow>
  );
}

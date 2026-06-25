import { IconLock } from "@tabler/icons-react";
import { TableCell, TableRow } from "../table";
import { Alert, AlertDescription, AlertTitle } from "../alert";
import { Button } from "../button";
import type { DataTableStateProps } from "@/types/data-table";

interface DataTableDisabledStateProps extends DataTableStateProps {
  message?: string;
  actionLabel?: string;
  onAction?: () => void;
}

export function DataTableDisabledState({
  colSpan,
  message = "La tabla está deshabilitada. Realiza una acción para continuar.",
  actionLabel,
  onAction,
}: DataTableDisabledStateProps) {
  return (
    <TableRow>
      <TableCell colSpan={colSpan} className="h-24 text-center">
        <Alert className="max-w-md mx-auto shadow-sm border-muted-foreground/20 bg-muted/50">
          <IconLock className="h-5 w-5 text-muted-foreground" />
          <AlertTitle className="font-medium text-muted-foreground">
            Tabla deshabilitada
          </AlertTitle>
          <AlertDescription className="flex flex-col gap-3 text-muted-foreground">
            <span>{message}</span>
            {actionLabel && onAction && (
              <Button
                onClick={onAction}
                variant="outline"
                size="sm"
                className="w-fit border-muted-foreground/30 hover:bg-muted/80"
              >
                {actionLabel}
              </Button>
            )}
          </AlertDescription>
        </Alert>
      </TableCell>
    </TableRow>
  );
}

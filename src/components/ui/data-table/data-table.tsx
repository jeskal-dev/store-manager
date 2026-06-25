import { flexRender } from "@tanstack/react-table";
import { useMemo } from "react";
import { DataTableEmptyState } from "./data-table-empty-state";
import { DataTableErrorState } from "./data-table-error-state";
import { DataTableLoadingState } from "./data-table-loading-state";
import type { Table as TanstackTable } from "@tanstack/react-table";
import type React from "react";

import { DataTablePagination } from "./data-table-pagination";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "../table";
import { cn } from "@/lib/utils";
import { getColumnPinningStyle } from "@/lib/data-table";

interface QueryStateProps {
  /** Indica si los datos están cargando */
  isDisabled?: boolean;
  isLoading?: boolean;
  /** Indica si hubo un error en la petición */
  isError?: boolean;
  /** Objeto de error de la petición */
  error?: Error | null;
  /** Indica si no hay datos disponibles (útil para estado vacío post-carga) */
  isEmpty?: boolean;
}

interface DataTableProps<TData>
  extends React.ComponentProps<"div">, QueryStateProps {
  table: TanstackTable<TData>;
  actionBar?: React.ReactNode;
  /** Texto personalizado para estado vacío */
  emptyMessage?: string;
  /** Texto personalizado para estado de carga */
  loadingMessage?: string;
  /** Texto personalizado para estado de error */
  errorMessage?: string;
  /** Componente personalizado para estado de carga */
  loadingComponent?: React.ReactNode;
  /** Componente personalizado para estado de error */
  errorComponent?: React.ReactNode;
  /** Componente personalizado para estado vacío */
  emptyComponent?: React.ReactNode;
  disabledComponent?: React.ReactNode;
  /** Callback para reintentar en caso de error (útil con useQuery.refetch) */
  onRetry?: () => void;
}

export function DataTable<TData>({
  table,
  actionBar,
  children,
  className,
  // Estados de query
  isDisabled = false,
  isLoading = false,
  isError = false,
  error = null,
  isEmpty = false,
  // Mensajes personalizables
  emptyMessage,
  loadingMessage,
  errorMessage,
  // Componentes personalizables
  loadingComponent,
  errorComponent,
  emptyComponent,
  disabledComponent,
  // Callbacks
  onRetry,
  ...props
}: DataTableProps<TData>) {
  const colSpan = table.getAllColumns().length;

  // Determinar el estado actual de forma priorizada
  const renderState = useMemo(() => {
    if (isDisabled) return "disabled";
    if (isLoading) return "loading";
    if (isError) return "error";
    if (isEmpty || table.getRowModel().rows.length === 0) return "empty";
    return "data";
  }, [isLoading, isError, isEmpty, isDisabled, table]);

  return (
    <div
      className={cn("flex w-full flex-col gap-2.5 overflow-auto", className)}
      {...props}
    >
      {children}

      <div className="overflow-hidden rounded-md border">
        <Table>
          <TableHeader>
            {table.getHeaderGroups().map((headerGroup) => (
              <TableRow key={headerGroup.id}>
                {headerGroup.headers.map((header) => (
                  <TableHead
                    key={header.id}
                    colSpan={header.colSpan}
                    style={{
                      ...getColumnPinningStyle({ column: header.column }),
                    }}
                  >
                    {header.isPlaceholder
                      ? null
                      : flexRender(
                          header.column.columnDef.header,
                          header.getContext(),
                        )}
                  </TableHead>
                ))}
              </TableRow>
            ))}
          </TableHeader>

          <TableBody>
            {renderState === "disabled" && disabledComponent}

            {renderState === "loading" &&
              (loadingComponent ?? (
                <DataTableLoadingState
                  colSpan={colSpan}
                  message={loadingMessage}
                />
              ))}

            {renderState === "error" &&
              (errorComponent ?? (
                <DataTableErrorState
                  colSpan={colSpan}
                  message={errorMessage}
                  error={error}
                  onRetry={onRetry}
                />
              ))}

            {renderState === "empty" &&
              (emptyComponent ?? (
                <DataTableEmptyState colSpan={colSpan} message={emptyMessage} />
              ))}

            {renderState === "data" &&
              table.getRowModel().rows.map((row) => (
                <TableRow
                  key={row.id}
                  data-state={row.getIsSelected() && "selected"}
                >
                  {row.getVisibleCells().map((cell) => (
                    <TableCell
                      key={cell.id}
                      style={{
                        ...getColumnPinningStyle({ column: cell.column }),
                      }}
                    >
                      {flexRender(
                        cell.column.columnDef.cell,
                        cell.getContext(),
                      )}
                    </TableCell>
                  ))}
                </TableRow>
              ))}
          </TableBody>
        </Table>
      </div>

      {/* Solo mostrar paginación si hay datos y no está cargando ni en error */}
      {renderState === "data" && (
        <div className="flex flex-col gap-2.5">
          <DataTablePagination table={table} />
          {actionBar &&
            table.getFilteredSelectedRowModel().rows.length > 0 &&
            actionBar}
        </div>
      )}
    </div>
  );
}

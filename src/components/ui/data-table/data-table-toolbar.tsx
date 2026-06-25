/* eslint-disable @typescript-eslint/no-unnecessary-condition */
import { IconX } from "@tabler/icons-react";
import type { Column, ColumnFiltersState, Table } from "@tanstack/react-table";
import { clone } from "es-toolkit";
import React, { useEffect, useState } from "react";

import { cn } from "@/lib/utils";
import { Button } from "../button";
import { Input } from "../input";
import { DataTableDateFilter } from "./data-table-date-filter";
import { DataTableFacetedFilter } from "./data-table-faceted-filter";
import { DataTableSliderFilter } from "./data-table-slider-filter";
import { DataTableViewOptions } from "./data-table-view-options";

interface DataTableToolbarProps<TData> extends React.ComponentProps<"div"> {
  table: Table<TData>;
}

export function DataTableToolbar<TData>({
  table,
  children,
  className,
  ...props
}: DataTableToolbarProps<TData>) {
  const [search, setSearch] = useState("");
  const state = table.getState();
  const isFiltered = state.columnFilters.length > 0;

  const columns = table
    .getAllColumns()
    .filter((column) => column.getCanFilter());

  const hasGlobalFilter = table
    .getAllColumns()
    .some((item) => item.columnDef.meta?.isGlobalSearch);

  // Determina si hay filtros ajustables (los que se muestran como DataTableToolbarFilter)
  const hasAdjustableFilters = columns.length > 0;

  const onReset = () => {
    table.resetColumnFilters();
    setSearch("");
  };

  useEffect(() => {
    if (hasGlobalFilter) {
      const filters = table.getState().columnFilters;

      let withGlobal: ColumnFiltersState = [...filters];

      if (search) withGlobal = [...filters, { id: "global", value: search }];

      table.setColumnFilters(withGlobal);
    }
  }, [search, hasGlobalFilter]);

  return (
    <div
      role="toolbar"
      aria-orientation="horizontal"
      className={cn(
        "flex w-full items-start justify-between gap-2 p-1",
        className,
      )}
      {...props}
    >
      <div className="flex flex-1 flex-wrap items-center gap-2">
        {hasGlobalFilter && (
          <Input
            placeholder="Buscar..."
            value={search}
            onChange={(event) => setSearch(clone(event.target.value))}
            className={cn(
              "h-8",
              hasAdjustableFilters
                ? "w-50 lg:w-56" // ancho normal si hay filtros ajustables
                : "w-64 lg:w-80", // más ancho si no hay filtros ajustables
            )}
          />
        )}
        {columns.map((column) => (
          <DataTableToolbarFilter key={column.id} column={column} />
        ))}
        {isFiltered && (
          <Button
            aria-label="Reset filters"
            variant="outline"
            size="sm"
            className="border-dashed"
            onClick={onReset}
          >
            <IconX />
            Reiniciar
          </Button>
        )}
      </div>
      <div className="flex items-center gap-2">
        {children}
        <DataTableViewOptions table={table} align="end" />
      </div>
    </div>
  );
}
interface DataTableToolbarFilterProps<TData> {
  column: Column<TData>;
}

function DataTableToolbarFilter<TData>({
  column,
}: DataTableToolbarFilterProps<TData>) {
  {
    const columnMeta = column.columnDef.meta;

    const onFilterRender = React.useCallback(() => {
      if (!columnMeta?.variant) return null;

      switch (columnMeta.variant) {
        case "text":
          return (
            <Input
              placeholder={columnMeta.placeholder ?? columnMeta.label}
              value={(column.getFilterValue() as string) ?? ""}
              onChange={(event) => column.setFilterValue(event.target.value)}
              className="h-8 w-40 lg:w-56"
            />
          );

        case "number":
          return (
            <div className="relative">
              <Input
                type="number"
                inputMode="numeric"
                placeholder={columnMeta.placeholder ?? columnMeta.label}
                value={(column.getFilterValue() as string) ?? ""}
                onChange={(event) => column.setFilterValue(event.target.value)}
                className={cn("h-8 w-30", columnMeta.unit && "pr-8")}
              />
              {columnMeta.unit && (
                <span className="absolute top-0 right-0 bottom-0 flex items-center rounded-r-md bg-accent px-2 text-muted-foreground text-sm">
                  {columnMeta.unit}
                </span>
              )}
            </div>
          );

        case "range":
          return (
            <DataTableSliderFilter
              column={column}
              title={columnMeta.label ?? column.id}
            />
          );

        case "date":
        case "dateRange":
          return (
            <DataTableDateFilter
              column={column}
              title={columnMeta.label ?? column.id}
              multiple={columnMeta.variant === "dateRange"}
            />
          );

        case "select":
        case "multiSelect":
          return (
            <DataTableFacetedFilter
              column={column}
              title={columnMeta.label ?? column.id}
              options={columnMeta.options ?? []}
              multiple={columnMeta.variant === "multiSelect"}
            />
          );

        default:
          return null;
      }
    }, [column, columnMeta]);

    return onFilterRender();
  }
}

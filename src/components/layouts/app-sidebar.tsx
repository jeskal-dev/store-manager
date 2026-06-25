import { SIDEBAR_NAV } from "@/constants/sidebar";
import type { SidebarLink } from "@/types/navigation";
import { IconChevronRight, IconPackage } from "@tabler/icons-react";
import { Link, useLocation } from "@tanstack/react-router";
import type { ComponentProps } from "react";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "../ui/collapsible";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarMenuSub,
  SidebarMenuSubButton,
  SidebarMenuSubItem,
} from "../ui/sidebar";

interface SidebarNavItemProps {
  link: SidebarLink;
  currentPath: string;
}

function SidebarNavItem({ link, currentPath }: SidebarNavItemProps) {
  const isActive = (url?: string) => {
    if (!url) return false;
    if (url === "/") return currentPath === "/";
    return currentPath === url || currentPath.endsWith(`${url}`);
  };

  if (link.sublinks?.length)
    return (
      <Collapsible key={link.label}>
        <SidebarMenuItem>
          <CollapsibleTrigger asChild>
            <SidebarMenuButton
              tooltip={link.label}
              data-active={
                link.sublinks.some((s) => isActive(s.url)) || undefined
              }
            >
              {link.icon && <link.icon />}
              <span>{link.label}</span>
              <IconChevronRight className="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90" />
            </SidebarMenuButton>
          </CollapsibleTrigger>
          <CollapsibleContent>
            <SidebarMenuSub>
              {link.sublinks.map((sub) => (
                <SidebarMenuSubItem key={sub.label}>
                  <SidebarMenuSubButton
                    asChild
                    data-active={isActive(sub.url) || undefined}
                  >
                    <Link to={sub.url!} preload="viewport">
                      <span>{sub.label}</span>
                    </Link>
                  </SidebarMenuSubButton>
                </SidebarMenuSubItem>
              ))}
            </SidebarMenuSub>
          </CollapsibleContent>
        </SidebarMenuItem>
      </Collapsible>
    );

  return (
    <SidebarMenuItem key={link.label}>
      <SidebarMenuButton tooltip={link.label} asChild>
        <Link to={link.url!} data-active={isActive(link.url) || undefined}>
          {link.icon && <link.icon />}
          <span>{link.label}</span>
        </Link>
      </SidebarMenuButton>
    </SidebarMenuItem>
  );
}

export function AppSidebar(props: ComponentProps<typeof Sidebar>) {
  const location = useLocation();
  const currentPath = location.pathname;

  return (
    <Sidebar collapsible="icon" {...props}>
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton
              size="lg"
              className="group-data-[collapsible=icon]:p-2!"
            >
              <div className="flex aspect-square size-8 items-center justify-center rounded-md bg-primary text-primary-foreground">
                <IconPackage className="size-4" />
              </div>
              <div className="grid flex-1 text-left text-sm leading-tight">
                <span className="font-heading truncate font-semibold tracking-tight">
                  Store Manager
                </span>
                <span className="truncate text-[10px] text-muted-foreground">
                  Panel de control
                </span>
              </div>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarContent>
        {SIDEBAR_NAV.map((group) => (
          <SidebarGroup key={group.label}>
            <SidebarGroupLabel>{group.label}</SidebarGroupLabel>
            <SidebarMenu>
              {group.links.map((link) => (
                <SidebarNavItem
                  key={link.label}
                  link={link}
                  currentPath={currentPath}
                />
              ))}
            </SidebarMenu>
          </SidebarGroup>
        ))}
      </SidebarContent>
      <SidebarFooter />
    </Sidebar>
  );
}

import type { Icon } from "@tabler/icons-react";

export type SidebarSection = {
  label?: string;
  links: SidebarLink[];
};

export type SidebarLink = {
  label: string;
  url?: string;
  icon?: Icon;
  sublinks?: SidebarLink[];
};

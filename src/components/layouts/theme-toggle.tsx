import { IconMoon, IconSun } from "@tabler/icons-react";
import { useTheme } from "@/hooks/common/use-theme";
import { Button } from "../ui/button";

export function ThemeToggle() {
  const { isDark, toggleTheme } = useTheme();

  return (
    <Button
      variant="ghost"
      size="icon"
      onClick={toggleTheme}
      aria-label={isDark ? "Cambiar a modo claro" : "Cambiar a modo oscuro"}
      className="h-8 w-8 shrink-0"
    >
      {isDark ? (
        <IconSun className="h-4 w-4" />
      ) : (
        <IconMoon className="h-4 w-4" />
      )}
    </Button>
  );
}

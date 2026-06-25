export const useThemeTransition = () => {
  const startTransition = (updateFn: () => void) => {
    if ("startViewTransition" in document) {
      document.startViewTransition(updateFn);
    } else {
      updateFn();
    }
  };
  return { startTransition };
};

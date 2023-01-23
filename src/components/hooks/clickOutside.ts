export function clickOutside(node: HTMLElement, args: Array<string>) {
  const handleClick = (event: { target: any; defaultPrevented: any }) => {
    if (!node?.contains(event.target) && !event.defaultPrevented) {
      node.dispatchEvent(new CustomEvent("clickOutside"));
    }
  };

  args.forEach((eventName) => {
    document.addEventListener(eventName, handleClick, true);
  });

  return {
    destroy() {
      args.forEach((eventName) => {
        document.removeEventListener(eventName, handleClick, true);
      });
    },
  };
}

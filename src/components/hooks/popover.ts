import type { StrictModifiers } from "@popperjs/core";
import { createPopper } from "@popperjs/core/dist/umd/popper.js";
import type { ComponentProps, SvelteComponentTyped } from "svelte";

export interface PopupMenuProps {
  component: new (...args: any[]) => SvelteComponentTyped;
  positioning: PositionArgs;
  openingEvent: string;
  closingEvent: string;
  componentProps: ComponentProps<SvelteComponentTyped>;
}

export interface PositionArgs {
  placement: PopupPlacement;
  xOffset: number;
  yOffset: number;
}

type PopupPlacement =
  | "mouse"
  | "auto"
  | "auto-start"
  | "auto-end"
  | "top"
  | "top-start"
  | "top-end"
  | "bottom"
  | "bottom-start"
  | "bottom-end"
  | "right"
  | "right-start"
  | "right-end"
  | "left"
  | "left-start"
  | "left-end";

let nonce = 0;
let activeHiders = [];

function hideAll() {
  activeHiders.forEach((fn) => fn());
  activeHiders = [];
}

export function popover(node: Node, props: PopupMenuProps) {
  let myId = ++nonce;

  let popperInstance = null;
  let componentInstance = null;
  let renderedComponent = null;
  let mouseElement = null;
  let isActive = false;
  const id = "popover";

  const toggle = (event: MouseEvent) => {
    mouseElement = {
      getBoundingClientRect: generateGetBoundingClientRect(event.x, event.y),
    };

    event.stopPropagation();
    event.preventDefault();

    if (isActive) {
      hideSelf();
    } else {
      hideAll();
      showSelf();
    }
  };

  node.addEventListener(props.openingEvent, toggle);

  const detectClickOutside = (event: { target: any }) => {
    renderedComponent = document.getElementById(id + myId);
    if (
      renderedComponent &&
      !renderedComponent.contains(event.target) &&
      isActive &&
      !node.contains(event.target)
    ) {
      hideSelf();
    }
  };

  const showSelf = () => {
    isActive = true;

    componentInstance = new props.component({
      target: document.body,
      props: props.componentProps,
    });

    renderedComponent = document.body.lastElementChild;
    renderedComponent.id = id + myId;
    renderedComponent.style.zIndex = "10000";

    popperInstance = createPopper<StrictModifiers>(
      props.positioning.placement !== "mouse" ? node : mouseElement,
      renderedComponent,
      {
        placement:
          props.positioning.placement !== "mouse"
            ? props.positioning.placement
            : "bottom-start",
        modifiers: [
          {
            name: "offset",
            options: {
              offset: [props.positioning.yOffset, props.positioning.xOffset],
            },
          },
        ],
      }
    );

    document.addEventListener(props.closingEvent, detectClickOutside);

    new MutationObserver(function () {
      if (!document.body.contains(node)) {
        hideSelf();
        this.disconnect();
      }
    }).observe(node.parentElement, { childList: true, subtree: false });

    activeHiders.push(hideSelf);
  };

  const hideSelf = () => {
    isActive = false;

    if (popperInstance) {
      popperInstance.destroy();
      popperInstance = null;
    }

    if (componentInstance) {
      componentInstance.$destroy();
      componentInstance = null;
    }

    document.removeEventListener(props.closingEvent, detectClickOutside);
  };

  function generateGetBoundingClientRect(x = 0, y = 0) {
    return () => ({
      width: 0,
      height: 0,
      top: y,
      right: x,
      bottom: y,
      left: x,
    });
  }

  return {
    destroy() {
      node.removeEventListener(props.openingEvent, toggle);
    },
  };
}

<script lang="ts">
  import { onMount, onDestroy } from "svelte";

  export let leftPaneSize = "50%";
  export let rightPaneSize = "50%";
  export let minLeftPaneSize = "0";
  export let minRightPaneSize = "0";
  export let showSeparator: boolean;
  export let updateCallback = () => {};
  export let onMouseDown = () => {};
  export let onMouseUp = () => {};

  let separator: HTMLDivElement;
  let mouseDownInfo: {
    e: any;
    firstWidth: any;
    secondWidth: any;
    offsetLeft: any;
    offsetTop?: any;
  };
  let left: HTMLDivElement, right: HTMLDivElement;

  $: leftPaneSize && resetSize();
  $: rightPaneSize && resetSize();

  const onMouseDownWrapper = (e) => {
    e.preventDefault();
    onMouseDown();
    if (e.button !== 0) return;
    mouseDownInfo = {
      e,
      offsetLeft: separator.offsetLeft,
      offsetTop: separator.offsetTop,
      firstWidth: left.offsetWidth,
      secondWidth: right.offsetWidth,
    };
    window.addEventListener("mousemove", onMouseMove);
    window.addEventListener("mouseup", onMouseUpWrapper);
    window.addEventListener("touchmove", onMouseMove);
    window.addEventListener("touchend", onMouseUpWrapper);
  };

  const onMouseMove = (e) => {
    e.preventDefault();
    if (e.button !== 0) return;
    var delta = {
      x: e.clientX - mouseDownInfo.e.clientX,
      y: e.clientY - mouseDownInfo.e.clientY,
    };
    delta.x = Math.min(
      Math.max(delta.x, -mouseDownInfo.firstWidth),
      mouseDownInfo.secondWidth
    );
    separator.style.left = mouseDownInfo.offsetLeft + delta.x + "px";
    left.style.width = mouseDownInfo.firstWidth + delta.x + "px";
    right.style.width = mouseDownInfo.secondWidth - delta.x + "px";
    updateCallback();
  };

  const onMouseUpWrapper = (e) => {
    onMouseUp();
    if (e) {
      e.preventDefault();
      if (e.button !== 0) return;
    }
    updateCallback();
    window.removeEventListener("mousemove", onMouseMove);
    window.removeEventListener("mouseup", onMouseUpWrapper);
    window.removeEventListener("touchmove", onMouseMove);
    window.removeEventListener("touchend", onMouseUpWrapper);
  };

  function resetSize() {
    if (left) left.removeAttribute("style");
    if (right) right.removeAttribute("style");
    if (separator) separator.removeAttribute("style");
  }

  function onResize() {
    onMouseUpWrapper(null);
    resetSize();
  }

  onMount(() => {
    window.addEventListener("resize", onResize);
  });

  onDestroy(() => {
    window.removeEventListener("resize", onResize);
  });
</script>

<div
  class="wrapper"
  style="--left-panel-size: {leftPaneSize}; --right-panel-size: {rightPaneSize}; --min-left-panel-size: {minLeftPaneSize}; --min-right-panel-size: {minRightPaneSize};"
>
  <div bind:this={left} class="left">
    <slot name="left">
      <div style="background-color: red;">Left Contents goes here...</div>
    </slot>
  </div>
  <div
    bind:this={separator}
    class="separator"
    class:hidden={!showSeparator}
    on:mousedown={onMouseDownWrapper}
    on:touchstart={onMouseDownWrapper}
  />
  <div bind:this={right} class="right">
    <slot name="right">
      <div style="background-color: yellow;">Right Contents goes here...</div>
    </slot>
  </div>
</div>

<style>
  div.wrapper {
    width: 100%;
    height: 100%;
    display: inline-flex;
  }
  div.separator {
    cursor: col-resize;
    height: 100%;
    width: 4px;
    margin-left: -2px;
    background-color: #aaa;
    background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='10' height='30'><path d='M2 0 v30 M5 0 v30 M8 0 v30' fill='none' stroke='black'/></svg>");
    background-repeat: no-repeat;
    background-position: center;
  }
  div.left {
    width: var(--left-panel-size);
    min-width: var(--min-left-panel-size);
    height: 100%;
  }
  div.right {
    width: var(--right-panel-size);
    min-width: var(--min-right-panel-size);
    height: 100%;
  }
</style>

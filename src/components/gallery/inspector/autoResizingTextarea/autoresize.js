import getSizingData from "./get-sizing-data.js";
import calculateNodeHeight from "./calculate-node-height.js";

export default function autoResize(node, maxHeightPixels) {
  node.addEventListener("input", () => resizeManually(node, maxHeightPixels));
  window.addEventListener("resize", () =>
    resizeManually(node, maxHeightPixels)
  );

  resizeManually(node, maxHeightPixels);

  return {
    destroy() {
      node.removeEventListener("input", resizeManually, true);
      window.removeEventListener("resize", resizeManually, true);
    },
  };
}

export function resizeManually(node, maxHeightPixels) {
  const nodeSizingData = getSizingData(node);

  if (!nodeSizingData) {
    return;
  }

  const height = calculateNodeHeight(
    nodeSizingData,
    node.value || node.placeholder || "x"
  );

  if (height > maxHeightPixels) {
    return;
  }

  node.style.setProperty("height", `${height}px`, "important");
}

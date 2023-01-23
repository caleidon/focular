<script lang="ts" context="module">
  import { get } from "svelte/store";
  import type { Metadata } from "../../../types/Metadata";
  import {
    orderingSettings,
    Ordering,
    OrderingDirection,
  } from "../../../code/filterStore";
  import { getFileSize } from "../../../code/ContentManager";

  export async function orderMetadata(metadata: Metadata[]) {
    if (get(orderingSettings).direction === OrderingDirection.Relevance) {
      return metadata;
    }

    switch (get(orderingSettings).orderBy) {
      case Ordering.Name:
        metadata = metadata.sort((a, b) => a.name.localeCompare(b.name));
        break;
      case Ordering.Extension:
        metadata = metadata.sort((a, b) => {
          let resultByExtension = a.extension.localeCompare(b.extension);
          return resultByExtension === 0
            ? a.name.localeCompare(b.name)
            : resultByExtension;
        });
        break;
      case Ordering.FileSize:
        let result = await getFileSizes(metadata);

        metadata = metadata.sort((a, b) => {
          let aSize = result[a.hash];
          let bSize = result[b.hash];
          return aSize === bSize ? a.name.localeCompare(b.name) : aSize - bSize;
        });

        break;
      case Ordering.DateAdded:
        metadata = metadata.sort(
          (a, b) => a.timestamp_created - b.timestamp_created
        );
        break;
      case Ordering.DateModified:
        metadata = metadata.sort(
          (a, b) => a.timestamp_modified - b.timestamp_modified
        );
        break;
    }

    // if we're dealing with dates, ascending is actually descending (show newest first)
    // TODO: can we do this more nicely?
    switch (get(orderingSettings).direction) {
      case OrderingDirection.Ascending:
        return get(orderingSettings).orderBy === Ordering.DateAdded ||
          get(orderingSettings).orderBy === Ordering.DateModified
          ? metadata.reverse()
          : metadata;
      case OrderingDirection.Descending:
        return get(orderingSettings).orderBy === Ordering.DateAdded ||
          get(orderingSettings).orderBy === Ordering.DateModified
          ? metadata
          : metadata.reverse();
    }
  }

  async function getFileSizes(metadatas: Array<Metadata>) {
    let fileSizes: { [name: string]: number } = {};
    for (let metadata of metadatas) {
      let size = await getFileSize(metadata.path);
      fileSizes[metadata.hash] = size;
    }
    return fileSizes;
  }
</script>

<script lang="ts">
  function updateOrderingFilter(
    e: Event & {
      currentTarget: EventTarget & HTMLInputElement;
    }
  ) {
    $orderingSettings.orderBy = +e.currentTarget.value;
  }
</script>

<!-- TODO: extract into separate components -->
<div class="bg-gray-700">
  <div class="btn-group">
    <button
      class="btn btn-sm"
      class:btn-active={$orderingSettings.direction ===
        OrderingDirection.Relevance}
      on:click={() =>
        ($orderingSettings.direction = OrderingDirection.Relevance)}
      >Relevance</button
    >
    <button
      class="btn btn-sm"
      class:btn-active={$orderingSettings.direction ===
        OrderingDirection.Ascending}
      on:click={() =>
        ($orderingSettings.direction = OrderingDirection.Ascending)}
      >Ascending</button
    >
    <button
      class="btn btn-sm"
      class:btn-active={$orderingSettings.direction ===
        OrderingDirection.Descending}
      on:click={() =>
        ($orderingSettings.direction = OrderingDirection.Descending)}
      >Descending</button
    >
  </div>

  <label class="label cursor-pointer">
    <span class="label-text">Name</span>
    <input
      type="radio"
      name="order-filter"
      class="radio"
      value={Ordering.Name}
      checked={$orderingSettings.orderBy === Ordering.Name}
      on:change={updateOrderingFilter}
      disabled={$orderingSettings.direction === OrderingDirection.Relevance}
    />
  </label>
  <label class="label cursor-pointer">
    <span class="label-text">Extension</span>
    <input
      type="radio"
      name="order-filter"
      class="radio"
      value={Ordering.Extension}
      checked={$orderingSettings.orderBy === Ordering.Extension}
      on:change={updateOrderingFilter}
      disabled={$orderingSettings.direction === OrderingDirection.Relevance}
    />
  </label>
  <label class="label cursor-pointer">
    <span class="label-text">Size</span>
    <input
      type="radio"
      name="order-filter"
      class="radio"
      value={Ordering.FileSize}
      checked={$orderingSettings.orderBy === Ordering.FileSize}
      on:change={updateOrderingFilter}
      disabled={$orderingSettings.direction === OrderingDirection.Relevance}
    />
  </label>
  <label class="label cursor-pointer">
    <span class="label-text">Date added</span>
    <input
      type="radio"
      name="order-filter"
      class="radio"
      value={Ordering.DateAdded}
      checked={$orderingSettings.orderBy === Ordering.DateAdded}
      on:change={updateOrderingFilter}
      disabled={$orderingSettings.direction === OrderingDirection.Relevance}
    />
  </label>
  <label class="label cursor-pointer">
    <span class="label-text">Date modified</span>
    <input
      type="radio"
      name="order-filter"
      class="radio"
      value={Ordering.DateModified}
      checked={$orderingSettings.orderBy === Ordering.DateModified}
      on:change={updateOrderingFilter}
      disabled={$orderingSettings.direction === OrderingDirection.Relevance}
    />
  </label>
</div>

<script lang="ts" context="module">
  import {
    dateFilterSettings,
    DateFilterMode,
    DateFilterRange,
  } from "../../../../code/filterStore";
  import { get } from "svelte/store";
  import type { Metadata } from "../../../../types/Metadata";

  export function resetDateFilter() {
    dateFilterSettings.set({
      dateRange: DateFilterRange.All,
      sortBy: DateFilterMode.Added,
    });
  }

  export function filterMetadataByDate(metadata: Metadata[]) {
    return metadata.filter((m) => {
      let currentTimestamp = Math.floor(Date.now() / 1000);
      let unix_day = 86400;
      let maxValidTimestamp: number;
      switch (get(dateFilterSettings).dateRange) {
        case DateFilterRange.Today:
          maxValidTimestamp = currentTimestamp - unix_day;
          break;
        case DateFilterRange.Yesterday:
          maxValidTimestamp = currentTimestamp - unix_day * 2;
          break;
        case DateFilterRange.LastWeek:
          maxValidTimestamp = currentTimestamp - unix_day * 7;
          break;
        case DateFilterRange.Last30Days:
          maxValidTimestamp = currentTimestamp - unix_day * 30;
          break;
        case DateFilterRange.Last90Days:
          maxValidTimestamp = currentTimestamp - unix_day * 90;
          break;
        case DateFilterRange.Last180Days:
          maxValidTimestamp = currentTimestamp - unix_day * 180;
          break;
        case DateFilterRange.Last365Days:
          maxValidTimestamp = currentTimestamp - unix_day * 365;
          break;
      }

      let validTimestamp =
        get(dateFilterSettings).sortBy === DateFilterMode.Added
          ? m.timestamp_created >= maxValidTimestamp
          : m.timestamp_modified >= maxValidTimestamp;

      return validTimestamp;
    });
  }

  export function isDateFilterEnabled() {
    return get(dateFilterSettings).dateRange !== DateFilterRange.All;
  }
</script>

<script lang="ts">
  function updateDateRange(
    e: Event & {
      currentTarget: EventTarget & HTMLInputElement;
    }
  ) {
    $dateFilterSettings.dateRange = +e.currentTarget.value;
  }
</script>

<div class="bg-gray-700">
  <div class="btn-group">
    <button
      class="btn btn-sm"
      class:btn-active={$dateFilterSettings.sortBy === DateFilterMode.Added}
      on:click={() => ($dateFilterSettings.sortBy = DateFilterMode.Added)}
      >Date added</button
    >
    <button
      class="btn btn-sm"
      class:btn-active={$dateFilterSettings.sortBy === DateFilterMode.Modified}
      on:click={() => ($dateFilterSettings.sortBy = DateFilterMode.Modified)}
      >Last modified</button
    >
  </div>

  <!-- TODO: extract into separate components -->
  <label class="label cursor-pointer">
    <span class="label-text">All</span>
    <input
      type="radio"
      name="date-filter-range"
      class="radio"
      value={DateFilterRange.All}
      checked={$dateFilterSettings.dateRange === DateFilterRange.All}
      on:change={updateDateRange}
    />
  </label>
  <label class="label cursor-pointer">
    <span class="label-text">Today</span>
    <input
      type="radio"
      name="date-filter-range"
      class="radio"
      value={DateFilterRange.Today}
      checked={$dateFilterSettings.dateRange === DateFilterRange.Today}
      on:change={updateDateRange}
    />
  </label>
  <label class="label cursor-pointer">
    <span class="label-text">Yesterday</span>
    <input
      type="radio"
      name="date-filter-range"
      class="radio"
      value={DateFilterRange.Yesterday}
      checked={$dateFilterSettings.dateRange === DateFilterRange.Yesterday}
      on:change={updateDateRange}
    />
  </label>
  <label class="label cursor-pointer">
    <span class="label-text">Last week</span>
    <input
      type="radio"
      name="date-filter-range"
      class="radio"
      value={DateFilterRange.LastWeek}
      checked={$dateFilterSettings.dateRange === DateFilterRange.LastWeek}
      on:change={updateDateRange}
    />
  </label>
  <label class="label cursor-pointer">
    <span class="label-text">Last month</span>
    <input
      type="radio"
      name="date-filter-range"
      class="radio"
      value={DateFilterRange.Last30Days}
      checked={$dateFilterSettings.dateRange === DateFilterRange.Last30Days}
      on:change={updateDateRange}
    />
  </label>
  <label class="label cursor-pointer">
    <span class="label-text">Last 90 days</span>
    <input
      type="radio"
      name="date-filter-range"
      class="radio"
      value={DateFilterRange.Last90Days}
      checked={$dateFilterSettings.dateRange === DateFilterRange.Last90Days}
      on:change={updateDateRange}
    />
  </label>
  <label class="label cursor-pointer">
    <span class="label-text">Last 180 days</span>
    <input
      type="radio"
      name="date-filter-range"
      class="radio"
      value={DateFilterRange.Last180Days}
      checked={$dateFilterSettings.dateRange === DateFilterRange.Last180Days}
      on:change={updateDateRange}
    />
  </label>
  <label class="label cursor-pointer">
    <span class="label-text">Last year</span>
    <input
      type="radio"
      name="date-filter-range"
      class="radio"
      value={DateFilterRange.Last365Days}
      checked={$dateFilterSettings.dateRange === DateFilterRange.Last365Days}
      on:change={updateDateRange}
    />
  </label>
  <!-- TODO: implement custom date range -->
  <!-- <label class="label cursor-pointer">
    <span class="label-text">CUSTOM TODO</span>
    <input type="radio" name="date-filter-range" class="radio" value={} />
  </label> -->
</div>

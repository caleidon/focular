import { ContentType } from "./../types/ContentType";
import { derived, writable, type Writable } from "svelte/store";

export enum DateFilterMode {
  Added,
  Modified,
}

export enum DateFilterRange {
  All,
  Today,
  Yesterday,
  LastWeek,
  Last30Days,
  Last90Days,
  Last180Days,
  Last365Days,
}

type TagFilterSettings = {
  selectedTags: string[];
};

type TypeFilterSettings = {
  shownTypes: Array<ContentType>;
};

type DateFilterSettings = {
  sortBy: DateFilterMode;
  dateRange: DateFilterRange;
};

type OrderingSettings = {
  orderBy: Ordering;
  direction: OrderingDirection;
};

export enum Ordering {
  Name,
  Extension,
  FileSize,
  DateAdded,
  DateModified,
}

export enum OrderingDirection {
  Relevance,
  Ascending,
  Descending,
}

export const tagFilterSettings: Writable<TagFilterSettings> = writable({
  selectedTags: [],
});
export const typeFilterSettings: Writable<TypeFilterSettings> = writable({
  shownTypes: [
    ContentType.Image,
    ContentType.Gif,
    ContentType.Video,
    ContentType.Audio,
    ContentType.Link,
    ContentType.Other,
  ],
});
export const dateFilterSettings: Writable<DateFilterSettings> = writable({
  sortBy: DateFilterMode.Added,
  dateRange: DateFilterRange.All,
});
export const orderingSettings: Writable<OrderingSettings> = writable({
  orderBy: Ordering.Name,
  direction: OrderingDirection.Relevance,
});

export const orderingOrFiltersChanged = derived(
  [tagFilterSettings, typeFilterSettings, dateFilterSettings, orderingSettings],
  ([
    tagFilterSettings,
    typeFilterSettings,
    dateFilterSettings,
    orderingSettings,
  ]) => {
    return [
      tagFilterSettings,
      typeFilterSettings,
      dateFilterSettings,
      orderingSettings,
    ];
  }
);

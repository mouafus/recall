import {writable, type Writable} from 'svelte/store';
import type { ClipboardItem as GeneratedClipboardItem } from '$lib/bindings';

export type IClipboardItem = GeneratedClipboardItem;

class ClipboardManager {
    public history: Writable<Map<string, IClipboardItem>>;
    public order: Writable<string[]>;
    public searchQuery: Writable<string> = writable('');
    public selectedItem: Writable<IClipboardItem | undefined> = writable(undefined);

    constructor() {
        this.history = writable(new Map());
        this.order = writable([]);
        this.selectedItem = writable(undefined);
    }

    add(item: IClipboardItem) {
        this.history.update((map) => {
            const newMap = new Map(map);
            newMap.set(item.id, item);
            return newMap;
        });
        this.order.update((arr) => {
            const next = arr.filter((id) => id !== item.id);
            next.unshift(item.id);
            return next;
        });
    }

    set(items: IClipboardItem[]) {
        const newHistory = new Map<string, IClipboardItem>();
        const newOrder: string[] = [];
        for (const item of items) {
            newHistory.set(item.id, item);
            newOrder.push(item.id);
        }
        this.history.set(newHistory);
        this.order.set(newOrder);
    }

    promote(id: string) {
        this.order.update((arr) => {
            const next = arr.slice();
            const index = next.indexOf(id);
            if (index > -1) {
                next.splice(index, 1);
            }
            next.unshift(id);
            return next;
        });
    }

}

export const clipboard = new ClipboardManager();


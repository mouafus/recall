import {writable, type Writable} from 'svelte/store';

export interface IClipboardItem {
    id: string;
    content: string;
    content_type: string;
    timestamp: number;
}

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
            map.set(item.id, item);
            return map;
        });
        this.order.update((arr) => [item.id, ...arr]);
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
            const index = arr.indexOf(id);
            if (index > -1) {
                arr.splice(index, 1);
            }
            arr.unshift(id);
            return arr;
        });
    }

}

export const clipboard = new ClipboardManager();


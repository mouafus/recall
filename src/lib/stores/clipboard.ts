import {writable} from "svelte/store";

export interface IClipboardItem {
    id: string;
    content: string;
    content_type: string;
    timestamp: number;
}

export const clipboardStore = writable<IClipboardItem[]>([]);

export const addToHistory = (item: IClipboardItem) => {
    clipboardStore.update(curr =>{
        return [item, ...curr]
    });
}

export const setHistory = (items: IClipboardItem[]) => {
    clipboardStore.set(items);
}
<script lang="ts">
    import ClipboardItem from "./ClipboardItem.svelte";
    import {onMount} from "svelte";
    import { hide } from '@tauri-apps/api/app';
    import {clipboardStore, type IClipboardItem} from "$lib/stores/clipboard";
    import {invoke} from "@tauri-apps/api/core";

    export let onItemSelect = (_item: IClipboardItem) => {};

    let selectedIndex = 0;

    async function handleKeydown(event: KeyboardEvent) {
        if (event.key === 'ArrowDown') {
            event.preventDefault();
            selectedIndex = (selectedIndex + 1) % $clipboardStore.length;
            if ($clipboardStore[selectedIndex]) {
                onItemSelect($clipboardStore[selectedIndex]);
            }
        } else if (event.key === 'ArrowUp') {
            event.preventDefault();
            selectedIndex = (selectedIndex - 1 + $clipboardStore.length) % $clipboardStore.length;
            if ($clipboardStore[selectedIndex]) {
                onItemSelect($clipboardStore[selectedIndex]);
            }
        }
        else if (event.key === 'Enter') {
            event.preventDefault();
            await handlePasteItem()
        }
    }

    async function handlePasteItem() {
        if($clipboardStore[selectedIndex]){
            await invoke('paste_item', {id: $clipboardStore[selectedIndex].id});
        }
        await hide();
    }

    function selectItem(index: number) {
        selectedIndex = index;
        onItemSelect($clipboardStore[index]);
    }

    onMount(() => {
        if ($clipboardStore.length > 0) {
            onItemSelect($clipboardStore[selectedIndex]);
        }

        document.addEventListener('keydown', handleKeydown);
        return () => {
            document.removeEventListener('keydown', handleKeydown);
        };
    });

    $: if (selectedIndex >= $clipboardStore.length && $clipboardStore.length > 0) {
        selectedIndex = 0;
    }
</script>

<div class="border-r border-r-gray-400 w-2/5 px-2 overflow-scroll pb-6 select-none">
    <div class="h-4"></div>
    {#each $clipboardStore as item, index}
        <ClipboardItem
                selected={selectedIndex === index}
                tabindex={selectedIndex === index ? 0 : -1}
                item={item}
                onSelect={() => selectItem(index)}
        />
    {/each}
    <div class="h-4"></div>
</div>
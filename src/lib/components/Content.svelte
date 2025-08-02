<script lang="ts">
    import {derived} from "svelte/store";
    import {clipboard, type IClipboardItem} from "$lib/state";
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {hide} from "@tauri-apps/api/app";

    let selectedIndex = 0;
    $: clipboardItemList = derived([clipboard.order, clipboard.history], ([$order, $history]) =>
        $order.map(id => $history.get(id)).filter(Boolean) as IClipboardItem[]
    );

    $: selectedItem = $clipboardItemList[selectedIndex] || null;

    async function handleKeydown(event: KeyboardEvent) {
        if (event.key === 'ArrowDown') {
            event.preventDefault();
            selectedIndex = (selectedIndex + 1) % $clipboardItemList.length;
            if ($clipboardItemList[selectedIndex]) {
                selectedItem = $clipboardItemList[selectedIndex];
            }
        } else if (event.key === 'ArrowUp') {
            event.preventDefault();
            selectedIndex = (selectedIndex - 1 + $clipboardItemList.length) % $clipboardItemList.length;
            if ($clipboardItemList[selectedIndex]) {
                selectedItem = $clipboardItemList[selectedIndex];
            }
        } else if (event.key === 'Enter') {
            event.preventDefault();
            await handlePasteItem()
        }
    }

    async function handlePasteItem() {
        if ($clipboardItemList[selectedIndex]) {
            const itemId = $clipboardItemList[selectedIndex].id;
            await invoke('paste_item', {id: itemId});
            clipboard.promote(itemId)
        }
        await hide();
    }


    onMount(() => {
        if ($clipboardItemList.length > 0) {
            selectedItem = $clipboardItemList[0];
        }

        document.addEventListener('keydown', handleKeydown);
        return () => {
            document.removeEventListener('keydown', handleKeydown);
        };
    });

    function parseText(text?: string): string {
        if (!text) {
            return '';
        }
        return text.split('\n')[0].trim();
    }


</script>

<section class="w-full flex h-full">
    <div class="border-r border-r-gray-400 w-2/5 px-2 overflow-scroll pb-6 select-none">
        <div class="h-4"></div>
        {#each $clipboardItemList as item, index}
            <button
                    class="py-2 px-1.5 text-sm rounded-md text-white w-full {selectedItem.id === item.id ? 'bg-[#323335]' : ''} text-left truncate select-none"
                    tabindex={index}
                    on:click={() => {
            if(item){
                selectedItem= item;
            }
        }}
            >
                {parseText(item?.content)}
            </button>
        {/each}
        <div class="h-4"></div>
    </div>
    <div class="w-3/5 px-2 pt-2 pb-6 h-full overflow-y-auto select-text">
        {#if selectedItem}
            <p class="whitespace-pre-wrap text-xs">{selectedItem.content}</p>
        {/if}
        <div class="h-4"></div>
    </div>
</section>


<style>
    button {
        outline: none;
    }
</style>
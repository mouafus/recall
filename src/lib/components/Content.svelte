<script lang="ts">
    import {derived} from "svelte/store";
    import {clipboard, type IClipboardItem} from "$lib/state";
    import {onMount, onDestroy} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {hide} from "@tauri-apps/api/app";

    let selectedIndex = 0;
    $: clipboardItemList = derived([clipboard.order, clipboard.history, clipboard.searchQuery], ([$order, $history, $searchQuery]) => {
        const items = $order.map(id => $history.get(id)).filter(Boolean) as IClipboardItem[];

        if (!$searchQuery) return items;

        const query = $searchQuery.toLowerCase();
        return items.filter(item =>
            item.content && item.content.toLowerCase().includes(query)
        );
    });

    $: {
        if (selectedIndex >= $clipboardItemList.length && $clipboardItemList.length > 0) {
            selectedIndex = 0;
        }
    }

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
        const firstLine = text.split('\n').find(line => line.trim().length > 0) || '';
        if (firstLine.length > 50) {
            return firstLine.slice(0, 50) + '...';
        }
        return text;
    }

    let canvasEl: HTMLCanvasElement | null = null;

    $: if (selectedItem && selectedItem.content_type.startsWith('image') && selectedItem.image_base64 && canvasEl) {
        try {
            const w = selectedItem.image_width ?? 0;
            const h = selectedItem.image_height ?? 0;
            if (w > 0 && h > 0) {
                const bytes = Uint8Array.from(atob(selectedItem.image_base64), c => c.charCodeAt(0));
                const ctx = canvasEl.getContext('2d');
                if (ctx) {
                    canvasEl.width = w;
                    canvasEl.height = h;
                    const imageData = new ImageData(new Uint8ClampedArray(bytes), w, h);
                    ctx.putImageData(imageData, 0, 0);
                }
            }
        } catch (e) {
            // ignore drawing errors
        }
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
            {#if selectedItem.content_type.startsWith('image') && selectedItem.image_base64}
                <canvas bind:this={canvasEl} class="max-w-full h-auto border border-gray-500 rounded"/>
            {:else}
                <p class="whitespace-pre-wrap text-xs">{selectedItem.content}</p>
            {/if}
        {/if}
        <div class="h-4"></div>
    </div>
</section>


<style>
    button {
        outline: none;
    }
    canvas { display: block; }
</style>
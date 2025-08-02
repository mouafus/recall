<script lang="ts">
    import ClipboardHistory from "./ClipboardHistory.svelte";
    import ClipboardItemDetail from "./ClipboardItemDetail.svelte";
    import type {IClipboardItem} from "$lib/stores/clipboard.js";


    let selectedItem: IClipboardItem | undefined = undefined;

    function handleItemSelect(item: IClipboardItem) {
        selectedItem = item;
    }
</script>

<section class="w-full flex h-full">
    <ClipboardHistory onItemSelect={handleItemSelect}/>
    <ClipboardItemDetail item={selectedItem}/>
</section>
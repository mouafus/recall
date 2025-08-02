<script lang="ts">
    import type {IClipboardItem} from "$lib/stores/clipboard";

    export let item: IClipboardItem | undefined;
</script>

<div class="w-3/5 px-2 pt-2 pb-6 h-full overflow-y-auto select-text">
    {#if item}
        <p class="whitespace-pre-wrap text-xs">{item.content}</p>
    {/if}
    <div class="h-4"></div>
</div>
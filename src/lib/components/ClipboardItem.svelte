<script lang="ts">

    import type {IClipboardItem} from "$lib/stores/clipboard";

    export let tabindex: number = 0;
    export let selected: boolean = false;
    export let item: IClipboardItem

    export let onSelect: (id: string) => void = () => {
    };

    function parseText(text: string): string {
        return text.split('\n')[0].trim();
    }
</script>

<button
        class="py-2 px-1.5 text-sm rounded-md text-white w-full {selected ? 'bg-[#323335]' : ''} text-left truncate select-none"
        tabindex={tabindex}
        on:click={() => onSelect(item.id)}
>
    {parseText(item.content)}
</button>

<style>
    button {
        outline: none;
    }
</style>
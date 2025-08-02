<script lang="ts">
    import "../app.css";
    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";
    import {addToHistory, setHistory} from "$lib/stores/clipboard.js";
    import type {IClipboardItem} from "$lib/stores/clipboard.js";
    import {listen} from "@tauri-apps/api/event";
    import {hide} from "@tauri-apps/api/app";

    listen<IClipboardItem>('new-clipboard-item', (event) => {
        addToHistory(event.payload)
    });

    async function handleKeydown(event: KeyboardEvent) {
        if (event.key === 'Escape') {
            event.preventDefault();
            await hide();
        }
    }

    onMount(() => {
        invoke('get_history').then((result) => {
            setHistory(result as IClipboardItem[]);
        })

        listen("tauri://blur", async () => {
            await hide()
        });

        document.addEventListener('keydown', handleKeydown);
        return () => {
            document.removeEventListener('keydown', handleKeydown);
        };
    })


    let {children} = $props();
</script>

{@render children()}

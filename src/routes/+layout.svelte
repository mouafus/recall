<script lang="ts">
    import "../app.css";
    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";
    import { clipboard, type IClipboardItem } from '$lib/state';
    import {listen} from "@tauri-apps/api/event";

    listen<IClipboardItem>('new-clipboard-item', (event) => {
        clipboard.add(event.payload)
    });

    async function handleKeydown(event: KeyboardEvent) {
        if (event.key === 'Escape') {
            event.preventDefault();
            await invoke('hide_window');
        }
    }

    onMount(() => {
        invoke('get_history').then((result) => {
            clipboard.set(result as IClipboardItem[]);
        })

        listen("tauri://blur", async () => {
            await invoke('hide_window');
        });

        document.addEventListener('keydown', handleKeydown);
        return () => {
            document.removeEventListener('keydown', handleKeydown);
        };
    })


    let {children} = $props();
</script>

{@render children()}

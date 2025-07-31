<script lang="ts">
    import {onMount} from "svelte";
    import {Window} from "@tauri-apps/api/window";
    import {writeText} from '@tauri-apps/plugin-clipboard-manager';

    const appWindow = new Window('recall');

    let history: string[] = [];

    onMount(() => {
        appWindow.listen('tauri://blur', () => {
            appWindow.hide()
        });

        window.addEventListener('keydown', (e) => {
            console.log(e)
            if (e.key === 'Escape') {
                appWindow.hide();
            }
        })

        history = ["Texte copié 1", "https://tauri.app/", "Un autre élément"];

        return () => {
            window.removeEventListener('keydown', (e) => {
                if (e.key === 'Escape') {
                    appWindow.hide();
                }
            });
        };
    })

    async function pasteItem(item: string) {
        try {
            await writeText(item);
            console.log(`Copié dans le presse-papiers: ${item}`);
        } catch (err) {
            console.error(`Erreur lors de la copie: ${err}`);
        } finally {
            await appWindow.hide();
        }
    }
</script>

<main class="container">
    <input type="text" placeholder="🔍 Search in history..." class="search-bar"/>

    <ul class="history-list">
        {#each history as item}
            <li>
                <button on:click={() => pasteItem(item)}>{item}</button>
            </li>
        {/each}
    </ul>
</main>

<style>
    :global(body) {
        background-color: transparent;
    }

    .container {
        background: rgba(40, 40, 40, 0.9);
        color: white;
        font-family: sans-serif;
        border-radius: 12px;
        padding: 10px;
        border: 1px solid rgba(100, 100, 100, 0.5);
    }

    .search-bar {
        max-width: 100%;
        padding: 8px;
        margin-bottom: 10px;
        background: #333;
        border: 1px solid #555;
        color: white;
        border-radius: 6px;
    }

    .history-list {
        list-style: none;
        padding: 0;
        margin: 0;
    }

    .history-list li {
        padding: 8px;
        border-radius: 6px;
        display: flex;
        justify-content: space-between;
        cursor: pointer;
    }

    .history-list li:hover {
        background-color: #4a4a4a;
    }
</style>

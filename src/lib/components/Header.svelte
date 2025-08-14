<script lang="ts">
    import {clipboard} from "$lib/state";

    let searchValue = "";

    $: {
        clipboard.searchQuery.set(searchValue);
    }

    function handleKeydown(event: KeyboardEvent) {
        if ((event.metaKey || event.ctrlKey) && event.key === 'k') {
            event.preventDefault();
            document.getElementById('search-input')?.focus();
        }
    }
</script>

<svelte:window on:keydown={handleKeydown}/>

<div class="px-4 pb-2 select-none border-b border-b-gray-400 ">
    <div class="relative flex items-center w-full max-w-2xl mx-auto">
        <input
                autofocus
                id="search-input"
                type="text"
                bind:value={searchValue}
                placeholder="Saisir pour filtrer l'historique..."
                class="w-full py-2 outline-none text-gray-700 dark:text-gray-200"
        />

        <div class="absolute right-3 flex items-center space-x-1">
            <kbd class="px-1 py-0.5 text-xs rounded bg-gray-200/70 dark:bg-gray-700/70 text-gray-600 dark:text-gray-300">⌘K</kbd>
        </div>
    </div>
</div>
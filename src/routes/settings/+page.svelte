<script lang="ts">
    import {onMount} from "svelte";
    import {getSettings, updateSettings, type Settings} from "$lib/api/settings";

    let form: Settings = {max_items: 5000, shortcut: "CmdOrCtrl+Shift+V", autostart: true};
    let saving = false;
    let error: string | null = null;
    let saved = false;

    let fieldErrors: { shortcut: string; max_items: string } = {shortcut: '', max_items: ''};

    $: {
        const n = Number(form.max_items);
        if (!Number.isFinite(n) || !Number.isInteger(n)) {
            fieldErrors.max_items = 'Doit être un entier.';
        } else if (n < 2) {
            fieldErrors.max_items = 'Minimum 2.';
        } else {
            fieldErrors.max_items = '';
        }

        const sc = form.shortcut?.trim() || '';
        if (!sc) {
            fieldErrors.shortcut = 'Raccourci requis.';
        } else if (!sc.includes('+')) {
            fieldErrors.shortcut = 'Inclure au moins une combinaison (ex: CmdOrCtrl+Shift+V).';
        } else if (!/^[-A-Za-z0-9+]+$/.test(sc)) {
            fieldErrors.shortcut = 'Caractères non valides.';
        } else {
            fieldErrors.shortcut = '';
        }
    }

    $: formValid = !fieldErrors.shortcut && !fieldErrors.max_items;

    onMount(async () => {
        try {
            form = await getSettings();
        } catch (e) {
            console.error(e);
            error = "Impossible de charger les paramètres";
        }
    });

    async function submit(e: Event) {
        e.preventDefault();
        saved = false;
        error = null;

        if (!formValid) {
            error = 'Corrigez les erreurs du formulaire.';
            return;
        }

        saving = true;
        try {
            await updateSettings(form);
            saved = true;
        } catch (e) {
            console.error(e);
            error = "Échec de l'enregistrement. Vérifiez le raccourci (peut être déjà utilisé par le système).";
        } finally {
            saving = false;
        }
    }
</script>

<main class="min-h-screen bg-gray-900 text-gray-200">
    <div class="max-w-3xl mx-auto p-6">
        <div class="rounded-2xl bg-gray-800/80 border border-gray-700 shadow-xl">
            <form class="p-6 space-y-6" on:submit={submit} novalidate>
                <section class="space-y-2">
                    <div class="text-sm font-medium text-gray-300">Raccourci global</div>
                    <div class="rounded-xl border border-gray-700 bg-gray-800/60 p-4">
                        <div class="flex items-center gap-3">
                            <input
                                    type="text"
                                    bind:value={form.shortcut}
                                    class="w-72 px-3 py-2 rounded-md bg-gray-900/70 border border-gray-700 focus:outline-none focus:ring-2 {fieldErrors.shortcut ? 'focus:ring-red-500 border-red-500' : 'focus:ring-blue-500'}"
                                    placeholder="Ex: CmdOrCtrl+Shift+V"
                                    aria-invalid={fieldErrors.shortcut ? 'true' : 'false'}
                                    aria-describedby="shortcut-error"
                            />
                        </div>
                        {#if fieldErrors.shortcut}
                            <p id="shortcut-error" class="mt-2 text-xs text-red-400">{fieldErrors.shortcut}</p>
                        {:else}
                            <p class="mt-2 text-xs text-gray-400">ex: CmdOrCtrl+Shift+V.
                                Certains raccourcis peuvent être refusés par le système.</p>
                        {/if}
                    </div>
                </section>

                <div class="border-t border-gray-700"></div>

                <section class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div class="space-y-2">
                        <div class="text-sm font-medium text-gray-300">Nombre d'éléments conservés</div>
                        <div class="rounded-xl border border-gray-700 bg-gray-800/60 p-4">
                            <input
                                    type="number"
                                    bind:value={form.max_items}
                                    min="2"
                                    class="w-32 px-3 py-2 rounded-md bg-gray-900/70 border {fieldErrors.max_items ? 'border-red-500' : 'border-gray-700'} focus:outline-none focus:ring-2 {fieldErrors.max_items ? 'focus:ring-red-500' : 'focus:ring-blue-500'}"
                                    aria-invalid={fieldErrors.max_items ? 'true' : 'false'}
                                    aria-describedby="max-items-error"
                            />
                            {#if fieldErrors.max_items}
                                <p id="max-items-error" class="mt-2 text-xs text-red-400">{fieldErrors.max_items}</p>
                            {:else}
                                <p class="mt-2 text-xs text-gray-400">Limite la taille de l'historique.</p>
                            {/if}
                        </div>
                    </div>

                    <div class="space-y-2">
                        <div class="text-sm font-medium text-gray-300">Démarrage</div>
                        <div class="rounded-xl border border-gray-700 bg-gray-800/60 p-4 flex items-center gap-3">
                            <input id="auto" type="checkbox" bind:checked={form.autostart} class="h-4 w-4"/>
                            <label for="auto" class="text-sm">Lancer l'application au démarrage</label>
                        </div>
                    </div>
                </section>

                {#if error}
                    <div class="text-sm text-red-400">{error}</div>
                {/if}
                {#if saved}
                    <div class="text-sm text-green-400">Paramètres enregistrés.</div>
                {/if}

                <div class="pt-2 flex gap-3">
                    <button class="px-4 py-2 rounded-md bg-blue-600 hover:bg-blue-500 text-white text-sm disabled:opacity-60"
                            disabled={saving || !formValid}>
                        {saving ? "Enregistrement..." : "Enregistrer"}
                    </button>
                </div>
            </form>
        </div>
    </div>
</main>

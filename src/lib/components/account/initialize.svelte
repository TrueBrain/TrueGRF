<script lang="ts">
    import { createEventDispatcher } from "svelte";

    import { InlineLoading } from "carbon-components-svelte";

    import { getFile, getLatestCommit } from "$lib/helpers/github";

    const dispatch = createEventDispatcher();

    export let accessToken;
    export let project;

    let statusTree = "active";
    let statusDownload = "inactive";

    let totalFiles = 0;
    let totalDone = 0;

    async function cleanupStorage(db, filesSeen) {
        const transaction = db.transaction("files", "readwrite");
        const store = transaction.objectStore("files");

        /* Check if there are any files in the storage we haven't seen on the remote. */
        store.openCursor().onsuccess = function (event) {
            const cursor = event.target.result;
            if (!cursor) return;

            /* If this file wasn't seen, remove it from cache. */
            if (filesSeen.indexOf(cursor.value.path) === -1) {
                cursor.delete();
            }

            cursor.continue();
        };
    }

    async function initialize() {
        /* Get all the files on the remote. */
        let files = await getLatestCommit(accessToken, project);
        totalFiles = files.length;

        statusTree = "finished";
        statusDownload = "active";

        const request = indexedDB.open(project);

        /* Create the database if it doesn't exist yet. */
        request.onupgradeneeded = function () {
            const db = request.result;
            db.createObjectStore("files", { keyPath: "path" });
        };

        request.onsuccess = async function () {
            const db = request.result;
            const transaction = db.transaction("files");
            const store = transaction.objectStore("files");

            let filesSeen = [];

            /* Every time a file is done, we add to the counter and check if we have seen all files. */
            async function fileDone() {
                totalDone += 1;
                if (totalDone == totalFiles) {
                    statusDownload = "finished";
                    await cleanupStorage(db, filesSeen);
                    db.close();

                    dispatch("cached", filesSeen);
                }
            }

            /* For every file, check if we either have it, or fetch it from the GitHub API. */
            for (let file of files) {
                filesSeen.push(file.path);

                const request = store.get(file.path);
                request.onsuccess = async function () {
                    /* Check if we have this file already. */
                    if (request.result && request.result.sha == file.sha) {
                        await fileDone();
                        return;
                    }

                    /* File is not in store, so fetch it. */
                    let content = await getFile(accessToken, project, file.sha);
                    /* All files are base64 encoded. Decode everything except PNGs. */
                    if (!file.path.endsWith(".png")) {
                        content = atob(content);
                    } else {
                        /* GitHub API returns base64 encoded blobs with newlines and other stuff in it. */
                        content = btoa(atob(content));
                    }

                    /* Store the file in the database. */
                    const transaction = db.transaction("files", "readwrite");
                    const store = transaction.objectStore("files");
                    store.put({
                        path: file.path,
                        sha: file.sha,
                        content,
                    });

                    /* Wait till the transaction is acknowledged. */
                    transaction.oncomplete = async function () {
                        await fileDone();
                    };
                };
            }
        };
    }

    $: if (accessToken && project) initialize();
</script>

<div class="initialize">
    <InlineLoading status={statusTree} description="Fetching project tree ..." />
    <InlineLoading status={statusDownload} description="Downloading project ({totalDone} / {totalFiles} files) ..." />
</div>

<style>
    .initialize {
        padding-top: 20px;
    }
</style>

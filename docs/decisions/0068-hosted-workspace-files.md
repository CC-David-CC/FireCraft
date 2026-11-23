# Hosted WebOS Files integration

WebOS opens and saves FireCraft's editable `.fw.png` projects through the
existing browser editor and its native format reader and writer. The
same-origin WebOS mount uses the shared binary studio file controls, file
picker and revision-checked writer. Raster import places an image on the
active layer.

Each document replacement advances the shell's project identity so a new
or dropped project loses the previous Files binding. Document edits,
undo, redo and transaction rollback advance the session revision. A save
receipt clears the dirty flag only when both values still match the
snapshot. Opening a project through Files uses the editable-document
reader directly so a plain or damaged PNG cannot silently replace it
with a flattened raster.

The hosted entry uses an external module and relative asset paths. It
enables the Files bridge with `?webos`; the standalone page continues to
use its download, file picker and postMessage interfaces. The native
desktop continues to use its injected platform services.

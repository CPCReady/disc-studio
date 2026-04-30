// MIT License
// Copyright (c) Destroyer 2026.
const en = {
  // General
  app_name: 'xDSK Desktop',
  ready: 'Ready',
  loading: 'Loading…',
  error: 'Error',
  success: 'Success',
  cancel: 'Cancel',
  confirm: 'Confirm',
  close: 'Close',
  open: 'Open',
  save: 'Save',
  delete: 'Delete',
  add: 'Add',
  refresh: 'Refresh',
  yes: 'Yes',
  no: 'No',

  // Sidebar
  sidebar_title: 'DSK Files',
  sidebar_add_disk: 'Add DSK image',
  sidebar_no_disks: 'No disk images open',
  sidebar_no_disks_hint: 'Click + to open a DSK image',
  sidebar_close_disk: 'Close disk',
  sidebar_check: 'Check',
  sidebar_info: 'Info',
  sidebar_create: 'Create',
  sidebar_compare: 'Compare',

  // Topbar
  topbar_disc_ready: 'xdsk ready',
  topbar_disc_not_found: 'xdsk not found',
  topbar_console: 'Log',
  topbar_settings: 'Settings',
  topbar_theme: 'Theme',
  topbar_create_dsk: 'Create DSK',
  topbar_compare: 'Compare',

  // Disk tab bar
  tab_no_disk: 'No disk open',

  // Explorer
  explorer_name: 'Name',
  explorer_type: 'Type',
  explorer_size: 'Size',
  explorer_readonly: 'R/O',
  explorer_system: 'SYS',
  explorer_user: 'User',
  explorer_files: 'files',
  explorer_used: 'used',
  explorer_free: 'free',
  explorer_empty: 'Empty disk',
  explorer_empty_hint: 'This disk has no files.',
  explorer_no_disk: 'No disk image open',
  explorer_no_disk_hint: 'Add a DSK image using the + button in the sidebar.',
  explorer_import: 'Import',
  explorer_export: 'Export',
  explorer_remove: 'Remove',
  explorer_check: 'Check',
  explorer_info: 'Info',
  explorer_emulator: 'Emulator',
  explorer_load_addr: 'Load',
  explorer_exec_addr: 'Exec',
  explorer_dsk_path: 'Path',

  // Context menu
  ctx_view: 'View',
  ctx_export: 'Export…',
  ctx_remove: 'Remove…',
  ctx_attr_readonly: 'Read-Only',
  ctx_attr_system: 'System',
  ctx_user: 'User',
  ctx_execute: 'Execute in Emulator',

  // Bottom panel tabs
  panel_console: 'Log',
  panel_check: 'Check',
  panel_info: 'Info',
  panel_view: 'View',
  panel_create: 'Create',
  panel_compare: 'Compare',

  // Console
  console_clear: 'Clear',
  console_empty: 'No output yet.',

  // Export modal
  export_title: 'Export Files',
  export_dest: 'Destination folder',
  export_dest_placeholder: 'Select output directory…',
  export_browse: 'Browse',
  export_strip_header: 'Strip AMSDOS header',
  export_strip_header_desc: 'Export raw file data without the 128-byte AMSDOS header.',
  export_submit: 'Export',

  // Import modal
  import_title: 'Import Files',
  import_files: 'Files to import',
  import_add_files: 'Add files…',
  import_file_type: 'File type',
  import_file_type_auto: 'Auto',
  import_file_type_binary: 'Binary',
  import_file_type_ascii: 'ASCII',
  import_load_address: 'Load address (hex)',
  import_exec_address: 'Exec address (hex)',
  import_user: 'User number',
  import_read_only: 'Read-only',
  import_system: 'System file',
  import_force: 'Overwrite existing',
  import_submit: 'Import',

  // Remove modal
  remove_title: 'Remove Files',
  remove_message_single: 'Remove "{name}" from the disk?',
  remove_message_multi: 'Remove {n} files from the disk?',
  remove_warning: 'This action cannot be undone.',
  remove_confirm: 'Are you sure you want to remove these files?',
  remove_confirm_detail: 'This action cannot be undone.',
  remove_force: 'Force remove (ignore read-only)',
  remove_submit: 'Remove',

  // Copy modal
  copy_title: 'Copy to Disk',
  copy_dest: 'Destination DSK image',
  copy_dest_placeholder: 'Select destination DSK…',
  copy_browse: 'Browse',
  copy_force: 'Overwrite existing',
  copy_submit: 'Copy',

  // Check panel
  check_title: 'Disk Check',
  check_run: 'Run Check',
  check_no_disk: 'No disk selected',
  check_header: 'Header',
  check_directory: 'Directory',
  check_bitmap: 'Bitmap',
  check_ok: 'OK',
  check_passed: 'Check passed',
  check_failed: 'Check found issues',

  // Info panel
  info_title: 'Disk Info',
  info_run: 'Load Info',
  info_no_disk: 'No disk selected',
  info_format: 'Format',
  info_tracks: 'Tracks',
  info_sectors: 'Sectors/Track',
  info_sector_size: 'Sector size',
  info_capacity: 'Capacity',
  info_entries: 'Directory entries',
  info_blocks: 'Blocks',

  // View panel
  view_title: 'View File',
  view_select_file: 'Select a file from the explorer to view it.',
  view_format: 'View format',
  view_auto: 'Auto',
  view_basic: 'BASIC',
  view_hex: 'Hex',
  view_ascii: 'ASCII',
  view_disasm: 'Disassembly',
  view_run: 'View',

  // Create panel
  create_title: 'Create Disk',
  create_output: 'Output path',
  create_output_placeholder: 'Select output file…',
  create_browse: 'Browse',
  create_tracks: 'Tracks',
  create_sectors: 'Sectors/Track',
  create_force: 'Overwrite existing',
  create_submit: 'Create',
  create_path: 'Output path',
  create_tracks_hint: 'Default: 40',
  create_sectors_hint: 'Default: 9',
  create_force_hint: 'Overwrite if the file already exists',
  create_run: 'Create Disk',
  create_success: 'Disk created successfully',
  create_error: 'Failed to create disk',

  diff_results: 'Comparison Results',

  view_no_disk: 'Select a file in the explorer to view it',
  view_filename: 'Filename',
  view_output: 'Output',

  // Compare (diff) panel
  diff_title: 'Compare Disks',
  diff_disk_a: 'Disk A',
  diff_disk_b: 'Disk B',
  diff_browse: 'Browse',
  diff_run: 'Compare',
  diff_status_only_a: 'Only in A',
  diff_status_only_b: 'Only in B',
  diff_status_modified: 'Modified',
  diff_status_identical: 'Identical',
  diff_no_diff: 'Disks are identical',
  diff_disk_placeholder: 'Select DSK image…',
  diff_vs: 'vs',
  diff_no_results: 'No differences found',

  // Generic
  browse: 'Browse',
  export_confirm: 'Export',
  export_files_count: '{n} files',
  export_output_dir: 'Output folder',
  import_confirm: 'Import',
  import_browse: 'Click to add files…',
  import_hint: 'All file types supported',
  copy_confirm: 'Copy',
  copy_desc_single: 'Copy {name} to:',
  copy_desc_multi: 'Copy {n} files to:',
  copy_dest_disk: 'Destination disk',

  // Settings modal
  settings_title: 'Settings',
  settings_language: 'Language',
  settings_language_desc: 'Select the interface language.',
  settings_save: 'Save',
  settings_emulator_path: 'Emulator Path',
  settings_emulator_path_desc: 'Path to the RetroVirtualMachine executable.',
  settings_emulator_browse: 'Browse',
  settings_font_size: 'Font Size',
  settings_font_size_desc: 'Adjust the interface font size.',
  settings_font_size_sm: 'Small',
  settings_font_size_md: 'Medium',
  settings_font_size_lg: 'Large',

  // Right sidebar
  right_info: 'Information',
  right_section_disk: 'Disk',
  right_section_directory: 'Directory',
  right_section_blockmap: 'Block Map',
  right_verification: 'Verification',
  right_no_disk: 'No disk open',
} as const;

export type TranslationKey = keyof typeof en;
export default en;

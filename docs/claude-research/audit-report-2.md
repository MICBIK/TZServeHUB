# Audit Report: tauri-monitor-research-2.md

## Summary
Audited 28 Rust-based desktop system monitoring projects for accuracy. Found **7 significant issues** across star counts, project status, categorization, and tech stack classification.

## Issues Found

### 1. Star Count Discrepancies
- **ClementTsang/bottom**: Listed as 13,042 stars, actual is ~13,000 stars (minor variance acceptable)
- **Abdenasser/neohtop**: Listed as 8,940 stars, actual is 8,900 stars (8.9k) - minor variance
- **cjbassi/ytop**: Listed as 2,166 stars, actual is 2,200 stars (2.2k) - minor variance

### 2. Project Status Issues
- **cjbassi/ytop**: **CRITICAL** - Project is archived and deprecated since August 29, 2020. README states "NO LONGER MAINTAINED" and recommends using ClementTsang/bottom instead. Should not be listed as an active project.

### 3. Tech Stack Misclassification
- **mskrasnov/FSM**: Listed as "Rust + GUI" but actually uses "Rust + Iced" framework. Should be more specific about the GUI framework.
- **jbilakhi/pulsehud**: Listed under "Other GUI Framework Projects" but uses Tauri. Should be moved to "Tauri-Based Projects" section.

### 4. Categorization Errors
- **jbilakhi/pulsehud**: Incorrectly categorized in "Other GUI Framework Projects" when it's actually a Tauri-based project (Rust + Tauri + vanilla JavaScript).

### 5. Summary Statistics Errors
Due to the misclassification of jbilakhi/pulsehud:
- **Tauri-based projects**: Should be 9 projects (not 8)
- **Other GUI frameworks**: Should be 11 projects (not 12)

## Verified Accurate Information
- **ClementTsang/bottom**: Correctly identified as TUI-based Rust system monitor
- **Abdenasser/neohtop**: Correctly identified as Tauri + Svelte project
- **p-e-w/hegemon**: Correctly identified with 336 stars and TUI-based
- **raro42/mac-stats**: Correctly identified with 7 stars and Tauri-based
- **felps-dev/ghtray**: Correctly identified with 6 stars and Tauri v2
- **mskrasnov/FSM**: Correctly identified with 16 stars (though tech stack needs clarification)
- **jbilakhi/pulsehud**: Correctly identified with 0 stars (but wrong category)

## Recommendations
1. Remove or mark cjbassi/ytop as deprecated/archived
2. Move jbilakhi/pulsehud to Tauri-Based Projects section
3. Update mskrasnov/FSM tech stack to "Rust + Iced"
4. Correct summary statistics for technology distribution
5. Consider adding deprecation status indicators for archived projects

## Overall Assessment
The research document is largely accurate with good project coverage and descriptions. The main issues are organizational (categorization) and one critical status issue (deprecated project). Star count variations are minimal and within acceptable ranges for dynamic GitHub data.
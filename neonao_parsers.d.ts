/* tslint:disable */
export type LinkType = 
 | "Inline" 
 | "Autolink" 
 | "Email" 
 | "Reference" 
 | "ReferenceUnknown" 
 | "Collapsed" 
 | "CollapsedUnknown" 
 | "Shortcut" 
 | "ShortcutUnknown";

export type Alignment = 
 | "None" 
 | "Left" 
 | "Center" 
 | "Right";

export type Tag = 
 | { name: "Paragraph" } 
 | { name: "Emphasis" } 
 | { name: "Strong" } 
 | { name: "CodeBlock"; language: string } 
 | { name: "BlockQuote" } 
 | { name: "Strikethrough" } 
 | { name: "Table"; alignments: Alignment[] } 
 | { name: "TableHead" } 
 | { name: "TableRow" } 
 | { name: "TableCell" } 
 | { name: "FootnoteDefinition"; text: string } 
 | { name: "HtmlBlock" } 
 | { name: "Rule" } 
 | { name: "Item" } 
 | { name: "Header"; level: number } 
 | { name: "List"; first: number | null } 
 | { name: "Link"; kind: LinkType; url: string; title: string } 
 | { name: "Image"; kind: LinkType; url: string; title: string };

export type Event = 
 | { kind: "Start"; tag: Tag } 
 | { kind: "End"; tag: Tag } 
 | { kind: "Text"; text: string } 
 | { kind: "SoftBreak" } 
 | { kind: "Code"; code: string } 
 | { kind: "HardBreak" } 
 | { kind: "TaskListMarker"; checked: boolean } 
 | { kind: "Html"; html: string } 
 | { kind: "InlineHtml"; html: string } 
 | { kind: "FootnoteReference"; name: string };

export type Segment = { event: Event; range: [ number , number ] };

/**
* @param {string} source 
* @returns {Segment[]} 
*/
export function markdown(source: string): Segment[];

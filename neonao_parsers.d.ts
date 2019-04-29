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
 | { type: "Paragraph" } 
 | { type: "Emphasis" } 
 | { type: "Strong" } 
 | { type: "CodeBlock"; language: string } 
 | { type: "BlockQuote" } 
 | { type: "Strikethrough" } 
 | { type: "Table"; alignments: Alignment[] } 
 | { type: "TableHead" } 
 | { type: "TableRow" } 
 | { type: "TableCell" } 
 | { type: "FootnoteDefinition"; name: string } 
 | { type: "HtmlBlock" } 
 | { type: "Rule" } 
 | { type: "Item" } 
 | { type: "Header"; level: number } 
 | { type: "List"; first: number | null } 
 | { type: "Link"; linkType: LinkType; url: string; title: string } 
 | { type: "Image"; likeType: LinkType; url: string; title: string };

export type Event = 
 | { type: "Start"; tag: Tag } 
 | { type: "End"; tag: Tag } 
 | { type: "Text"; text: string } 
 | { type: "SoftBreak" } 
 | { type: "Code"; code: string } 
 | { type: "HardBreak" } 
 | { type: "TaskListMarker"; checked: boolean } 
 | { type: "Html"; html: string } 
 | { type: "InlineHtml"; html: string } 
 | { type: "FootnoteReference"; name: string };

export type Segment = { event: Event; range: [ number , number ] };

/**
* @param {string} source 
* @returns {Segment[]} 
*/
export function markdown(source: string): Segment[];

/* tslint:disable */
export type LinkType = 
 | "Inline" 
 | "Autolink" 
 | "Email" 
 | "Unsupported";

export type Tag = 
 | "Paragraph" 
 | "Emphasis" 
 | "Strong" 
 | "Code" 
 | "Unsupported" 
 | { Link: { kind: LinkType; url: string; title: string } };

export type Event = 
 | { Start: { tag: Tag } } 
 | { End: { tag: Tag } } 
 | { Text: { text: string } } 
 | "Unsupported";

export type Segment = { event: Event; range: [ number , number ] };

/**
* @param {string} source 
* @returns {Segment[]} 
*/
export function markdown(source: string): Segment[];

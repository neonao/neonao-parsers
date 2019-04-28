/* tslint:disable */
export type LinkType = 
 | "Inline" 
 | "Autolink" 
 | "Email" 
 | "Unsupported";

 export type Tag = 
 | { name: "Paragraph" } 
 | { name: "Emphasis" } 
 | { name: "Strong" } 
 | { name: "Code" } 
 | { name: "Unsupported" } 
 | { name: "Link"; kind: LinkType; url: string; title: string };

 export type Event = 
 | { kind: "Start"; tag: Tag } 
 | { kind: "End"; tag: Tag } 
 | { kind: "Text"; text: string } 
 | { kind: "Unsupported" };

export type Segment = { event: Event; range: [ number , number ] };

/**
* @param {string} source 
* @returns {Segment[]} 
*/
export function markdown(source: string): Segment[];

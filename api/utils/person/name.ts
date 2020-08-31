/**
 * Formats a name to acceptable alphanumeric, _, -
 * @param name The string to format
 * @example formatName('This name here') -> 'This-name-here'
 */
export const formatName = (name: string): string => name.replace(/[^\w-]/g, '-')

/**
 * Makes a common string out of another (distinct names)
 * @param name String to "localize"
 * @example makeReferenceName('This name here) -> 'thisnamehere'
 */
export const makeReferenceName = (name?: string): string => {
  if (name == null) {
    return ''
  }

  return formatName(name)
    .replace(/[-_\s]/g, '')
    .toLowerCase()
}

#define STB_IMAGE_IMPLEMENTATION

#include "blurhash_encode.h"
#include "stb_image.h"

const char *blur_hash_for_file(int xComponents, int yComponents,
                               const char *filename) {
  int width, height, channels;
  unsigned char *data = stbi_load(filename, &width, &height, &channels, 3);
  if (!data)
    return NULL;

  const char *hash = blurHashForPixels(xComponents, yComponents, width, height,
                                       data, width * 3);

  stbi_image_free(data);

  return hash;
}

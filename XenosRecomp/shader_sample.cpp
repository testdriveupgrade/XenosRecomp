#include "pch.h"
#include "shader.h"
#include "shader_sample.h"

#include <cstring>

namespace
{
    constexpr uint32_t kShaderHeaderSize = 24;
    constexpr uint32_t kContainerHeaderSize = 36;

    void write_be_u32(std::vector<uint8_t>& data, size_t offset, uint32_t value)
    {
        const uint32_t swapped = byteSwap(value);
        std::memcpy(data.data() + offset, &swapped, sizeof(swapped));
    }
}

std::vector<uint8_t> build_sample_shader_container()
{
    static_assert(sizeof(Shader) == kShaderHeaderSize);
    static_assert(sizeof(ShaderContainer) == kContainerHeaderSize);

    constexpr uint32_t shader_offset = kContainerHeaderSize;
    constexpr uint32_t virtual_size = kContainerHeaderSize + kShaderHeaderSize;
    constexpr uint32_t physical_size = 4;

    std::vector<uint8_t> data(virtual_size + physical_size, 0u);

    write_be_u32(data, 0, 0x102A1100);
    write_be_u32(data, 4, virtual_size);
    write_be_u32(data, 8, physical_size);
    write_be_u32(data, 12, 0);
    write_be_u32(data, 16, 0);
    write_be_u32(data, 20, 0);
    write_be_u32(data, 24, shader_offset);
    write_be_u32(data, 28, 0);
    write_be_u32(data, 32, 0);

    write_be_u32(data, shader_offset, 0);
    write_be_u32(data, shader_offset + 4, 1);
    write_be_u32(data, shader_offset + 8, 0);
    write_be_u32(data, shader_offset + 12, 0);
    write_be_u32(data, shader_offset + 16, 0);
    write_be_u32(data, shader_offset + 20, 0);

    return data;
}

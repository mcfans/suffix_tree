#include "arm_neon.h"
#include <_types/_uint64_t.h>
#include <cstdio>
#include <stdio.h>
#include <string.h>
#define ANKERL_NANOBENCH_IMPLEMENT
#include <nanobench.h>

extern "C" {
	inline __attribute__((always_inline)) int check_result(uint32x4_t result) {
	    uint16x4_t result16 = vshrn_n_u32(result, 16);
	    uint64x1_t u64 = vreinterpret_u64_u16(result16);
        uint64_t u64_ = vget_lane_u64(u64, 0);
	    int res = __builtin_clzll(u64_) >> 4;
	    return 3 - res;
	}

	inline __attribute__((always_inline)) int check_contains(char *pattern, char *text) {
	    int res;
	    uint8x16_t text_source = vld1q_u8((uint8_t *)text);

	    uint32x4_t pattern_source = vld1q_dup_u32((uint8_t *)pattern);

	    uint32x4_t result1 = vceqq_u32(vreinterpretq_u32_u8(text_source), pattern_source);

	    res = check_result(result1);
	    if (res >= 0) {
		return res;
	    }
	    return -1;
	}
}

int main(int argc, char **argv) {
    int res;
	char *pattern = argv[1];
	char *text = argv[2];
	unsigned long len = strlen(text);

    // ankerl::nanobench::Bench().run("some double ops", [&] {
		for (int j = 0; j < 100000000; j++) {
			for (int i = 0;i*32 < len;i++) {
				res = check_contains(pattern, text + i * 32 * sizeof(uint8_t));
				if (res >= 0) {
					break;
				}
			}
		}
    //     ankerl::nanobench::doNotOptimizeAway(res);
    // });
	ankerl::nanobench::doNotOptimizeAway(res);
}
// #include <benchmark/benchmark.h>

// // #define GCC_SPLIT_BLOCK(str)  __asm__( "//\n\t// " str "\n\t//\n" );

// // static void BM_SomeFunction(benchmark::State& state) {
// //   state.counters
// //   // Perform setup here
// //     char *pattern = "abcd";
// //     char *text = "badcabceabcdbadcbadcabcebadcbadcbadcabcebadcbadc";
// //   unsigned long len = strlen(text);
// //   int res;
  
// //   for (auto _ : state) {
// //     // This code gets timed
// //     for (int i = 0;i*32 < len;i++) {
// //         GCC_SPLIT_BLOCK("Call this please");
// //         benchmark::DoNotOptimize(res = check_contains(pattern, text + i * 32 * sizeof(uint8_t)));
// //         if (res >= 0) {
// //             break;
// //         }
// //     }
// //   }

// //   printf("res: %d\n", res);
// // }

// template <class ...Args>
// void BM_takes_args(benchmark::State& state, Args&&... args) {
//   auto args_tuple = std::make_tuple(std::move(args)...);
//   auto pattern = std::get<0>(args_tuple);
//   auto text = std::get<1>(args_tuple);

//   unsigned long len = strlen(text);

//   int res;
//   for (auto _ : state) {
//     for (int i = 0;i*32 < len;i++) {
//       res = check_contains(pattern, text + i * 32 * sizeof(uint8_t));
//       if (res >= 0) {
//         break;
//       }
//     }
//   }
// }
// // Registers a benchmark named "BM_takes_args/int_string_test" that passes
// // the specified values to `args`.
// BENCHMARK_CAPTURE(BM_takes_args, int_string_test, std::string("abcd"), str::string("badcabceabcdbadcbadcabcebadcbadcbadcabcebadcbadc"));
// // // Register the function as a benchmark
// // BENCHMARK(BM_SomeFunction);
// // Run the benchmark
// BENCHMARK_MAIN();

// // int main(int argc, char **argv) {
// //     char *pattern = argv[1];
// //     char *text = argv[2];
// //     unsigned long len = strlen(text);
// //     for (int j = 0; j < 1000000000; j++) {
// //         for (int i = 0;i*32 < len;i++) {
// //             int res = check_contains(pattern, text + i * 32 * sizeof(uint8_t));
// //             if (res >= 0) {
// //                 printf("res: %d\n", res);
// //                 return 0;
// //             }
// //         }
// //     }
    
// //     printf("res: -1");
// // }
